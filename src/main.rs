use base64::encode;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::string::String;
#[derive(Debug, Serialize)]
pub struct Pair {
    name: String,
    value: String,
}

fn get_pair_from_env(env: &str) -> Vec<Pair> {
    let env_str = match env::var(env) {
        Ok(e) => e,
        Err(_) => String::default(),
    };
    let envs: Vec<&str> = match env_str.as_str() {
        "" => Vec::new(),
        v => v.split('\n').collect(),
    };
    let mut re = Vec::<Pair>::new();
    if envs.len() > 0 {
        for e in envs {
            if e.trim() == "" {
                continue;
            }
            let ep: Vec<&str> = e.split('=').into_iter().collect();
            if ep.len() != 2 {
                panic!("cannot split {} by '='", e);
            }
            re.push(Pair {
                name: ep[0].trim().to_string(),
                value: ep[1].trim().to_string(),
            })
        }
    }
    re
}

fn get_env_string(env: &str, default: Option<String>) -> String {
    match env::var(env) {
        Ok(e) => e,
        Err(_) => default.unwrap_or_default(),
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    //portainer server url
    let server = env::var("INPUT_SERVERURL").unwrap();
    //portainer endpoint, default 1
    let endpoint = get_env_string("INPUT_ENDPOINTID", Some(String::from("1")));
    //stack content, content of docker-compose.yml
    let mut compose = get_env_string("INPUT_DOCKER_COMPOSE", None);
    let compose_path = get_env_string("INPUT_DOCKER_COMPOSE_PATH", None);
    let stack_name = env::var("INPUT_STACKNAME").unwrap();
    let api_key = get_env_string("INPUT_ACCESS_TOKEN", None);
    let username = get_env_string("INPUT_USERNAME", None);
    let password = get_env_string("INPUT_PASSWORD", None);
    if api_key == "" && (username == "" || password == "") {
        panic!("api_key and username or password cannot both empty");
    }
    let repo_username = get_env_string("INPUT_REPO_USERNAME", None);
    let repo_password = get_env_string("INPUT_REPO_PASSWORD", None);
    let images_str = get_env_string("INPUT_IMAGENAMES", None);
    let variables = get_pair_from_env("INPUT_VARIABLES");
    let envs = get_pair_from_env("INPUT_ENV");
    let client = reqwest::Client::new();

    //read content of compose_path to compose
    if compose == "" && compose_path != "" && variables.len() > 0 {
        compose = client
            .get(format!(
                "{}/raw/{}/{}",
                format!(
                    "https://github.com/{}",
                    env::var("GITHUB_REPOSITORY").unwrap()
                ),
                env::var("GITHUB_REF").unwrap(),
                compose_path
            ))
            .send()
            .await?
            .text()
            .await?;
    }
    //replace variables
    if variables.len() > 0 && compose != "" {
        for v in variables {
            compose = compose.replace(format!("{{{{ {} }}}}", v.name).as_str(), &v.value);
            compose = compose.replace(format!("{{{{{}}}}}", v.name).as_str(), &v.value);
        }
    }

    //1. login to portainer
    let mut auth_name = "X-API-Key";
    let mut auth_value = api_key;
    if auth_value == "" {
        auth_name = "Authorization";
        let login_result: serde_json::Value = client
            .post(format!("{}/api/auth", &server))
            .json(&serde_json::json!({
                "Username": &username,
                "Password": &password,
            }))
            .send()
            .await?
            .json()
            .await?;
        auth_value = format!("Bearer {}", &login_result["jwt"].as_str().unwrap());
    }

    let endpoint_result = client
        .get(format!("{}/endpoints/{}", &server, endpoint))
        .header(auth_name, &auth_value)
        .send()
        .await?;
    if endpoint_result.status() == reqwest::StatusCode::NOT_FOUND {
        panic!("can not found endpoint id is {} ", endpoint);
    }

    //2. pull image
    if images_str != "" {
        println!("pull images: {}", &images_str);
        //get all registry
        let registries: serde_json::Value = client
            .get(format!("{}/api/registries", &server))
            .header(auth_name, &auth_value)
            .send()
            .await?
            .json()
            .await?;
        let mut registy_map: HashMap<&str, i32> = HashMap::new();
        for r in registries.as_array().unwrap() {
            registy_map.insert(r["URL"].as_str().unwrap(), r["Id"].as_i64().unwrap() as i32);
        }

        let images: Vec<&str> = images_str.split('\n').collect();
        for image in images {
            if image.trim() == "" {
                continue;
            }
            let mut pull_image_header = reqwest::header::HeaderMap::new();
            pull_image_header.insert(auth_name, auth_value.parse().unwrap());
            let registry_name = image.split('/').nth(0).unwrap();
            //if image is in registry_map, pull it with X-Registry-Auth
            if registy_map.contains_key(registry_name) {
                let registry_id = registy_map[registry_name];
                let repo_auth = encode(format!("{{\"registryId\":{}}}", registry_id));
                pull_image_header.insert("X-Registry-Auth", repo_auth.parse().unwrap());
            }
            let pull_image_result = client
                .post(format!(
                    "{}/api/endpoints/{}/docker/images/create?fromImage={}",
                    &server,
                    &endpoint,
                    image.trim()
                ))
                .headers(pull_image_header)
                .send()
                .await?;
            if pull_image_result.status() == 200 {
                println!("pull image success : {}", image);
            } else {
                let msg_detail: serde_json::Value = pull_image_result.json().await?;
                println!("message:{}", &msg_detail["message"].as_str().unwrap());
            }
        }
    }

    //3. get stack id
    let stacks: serde_json::Value = client
        .get(format!(
            "{}/api/stacks?filters={{\"EndpointID\":{},\"IncludeOrphanedStacks\":true}}",
            &server, &endpoint
        ))
        .header(auth_name, &auth_value)
        .send()
        .await?
        .json()
        .await?;
    let mut stack_map: HashMap<&str, i32> = HashMap::new();
    for s in stacks.as_array().unwrap() {
        stack_map.insert(
            s["Name"].as_str().unwrap(),
            s["Id"].as_i64().unwrap() as i32,
        );
    }

    if stack_map.contains_key(stack_name.as_str()) {
        let stack_id = stack_map[stack_name.as_str()];
        println!("update stack id: {}", stack_id);
        // compose is empty, get original stack content
        if compose == String::default() {
            let compose_result: serde_json::Value = client
                .get(format!("{}/api/stacks/{}/file", &server, stack_id))
                .header(auth_name, &auth_value)
                .send()
                .await?
                .json()
                .await?;
            compose = compose_result["StackFileContent"]
                .as_str()
                .unwrap()
                .to_string();
        }
        //4. update stack
        let update_result: serde_json::Value = client
            .put(format!(
                "{}/api/stacks/{}?endpointId={}",
                &server, stack_id, endpoint
            ))
            .header(auth_name, auth_value)
            .json(&serde_json::json!({
                "id": stack_id,
                "StackFileContent": &compose,
                "Env": envs,
                "Prune": false}))
            .send()
            .await?
            .json()
            .await?;
        match update_result["message"].as_str() {
            Some(msg) => {
                println!("update stack failed: {}", msg);
                panic!("update stack failed");
            }
            None => println!("update stack success"),
        }
        return Ok(());
    }

    //5. create stack
    if compose == "" && compose_path == "" {
        panic!("compose is empty, cannot create stack");
    }
    if compose == "" {}
    //type: 0: docker compose, 1: docker stack
    //method: file string or repository
    let create_result: serde_json::Value = match compose.as_str() {
        "" => {
            client
                .post(format!(
                    "{}/api/stacks?endpointId={}&method=repository&type=2",
                    &server, endpoint
                ))
                .header(auth_name, auth_value)
                .json(&serde_json::json!({
                    "repositoryURL": env::var("GITHUB_REPOSITORY").unwrap(),
                    "repositoryReferenceName": env::var("GITHUB_REF").unwrap(),
                    "composeFile": compose_path,
                    "repositoryAuthentication": repo_password != "",
                    "repositoryUsername": repo_username,
                    "repositoryPassword": repo_password,
                    "Env": envs,
                    "Name": &stack_name,
                }))
                .send()
                .await?
                .json()
                .await?
        }

        c => {
            client
                .post(format!(
                    "{}/api/stacks?endpointId={}&method=string&type=2",
                    &server, endpoint
                ))
                .header(auth_name, auth_value)
                .json(&serde_json::json!({
                    "StackFileContent": c,
                    "Env": envs,
                    "Name": &stack_name,
                }))
                .send()
                .await?
                .json()
                .await?
        }
    };
    match create_result["message"].as_str() {
        Some(msg) => {
            println!("create stack failed: {}", msg);
            panic!("create stack failed");
        }
        None => println!("create stack success"),
    }

    Ok(())
}
