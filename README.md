# portainer-deploy
This action deploy image by portainer

# Inputs
---
## serverurl
**required**  portainer server url. like this: http://xxx.com:9000 <br>
portainer服务器公网地址  需要公网能够访问

## username
portainer username <br>
portainer 面板登录所用的用户名

## password
portainer password <br>
portainer 面板登录所用的密码

## access_token
portainer account [access token](https://docs.portainer.io/v/ce-2.11/api/access). login by username password or access_token.<br> 
click on my account in the top right -> Scroll down to the Access tokens section -> click the Add access token.<br>
可选 portainer账户的 [访问令牌](https://docs.portainer.io/v/ce-2.11/api/access), 登录方式二选一 用户名-密码 或者 访问令牌. <br>
创建访问令牌在 portainer 右上角 myAccount, 下拉到Access Token, 点击 Add access token,输入描述 确定

## endpointId
portainer endpoint id,default 1,  localhost is 1 <br>
portainer终结点id，默认是1,即第一个，一般为localhost

## stackname
**required** name of stack <br>
服务栈的名称，会在stacks列表里显示

## imagenames
names of pull images, a arrary. add this param because not auto pull image when image:tag not change in docker-compose <br>
可选 将会进行拉取镜像的镜像名列表, 为数组.加这个参数是因为docker-compose里的镜像名:tag 没有变化则不会自动拉取镜像

## env
environments of stack <br>
可选 环境变量列表 
![env](https://p.sda1.dev/5/b982dedaf195db23d1767701e4200ebd/msedge_xwrxILQuNN.webp)

## variables
these variables will be replaced in docker-compose file，  Foo=bar will replace {{ Foo }} for bar.<br>
变量列表，会自动替换docker-compose中的变量 如设定Foo=bar,  则会把docker-compose中的{{ Foo }}替换为bar

## docker_compose
content of docker-compose.yml.  it will be filled by original stack when stack exist. <br>
docker-compose.yml的内容 如果stack已经存在,不填则会自动获取已经存在的stack内容 <br>
sample like this:
```
docker_compose: |
  version: "3"
  services:
    dotnettest:
      image:  mcr.microsoft.com/dotnet/core/aspnet:6.0-alpine
      container_name: dotnet_runtime
```
<br>

## docker_compose_path
docker-compose.yml in repository relative path. just need choose one between docker_compose and docker_compose_path.
note: if stack exist, it will be use original content of stack, not content of docker-compose.yml in repository, so docker_compose_path can work when stack not exist <br>
可选, docker-comose.yml在git仓库中的相对路径, docker_compose和docker_compose_path二选一即可, 注意,如果stack已存在会先使用已存在的stack的内容,即docker_compose_path仅能用于创建stack的情况
    
## repo_username
username of git repository<br>
可选, git仓库用户名

## repo_password
password of git repository<br>
可选, git仓库密码
# Example usage
The following will pull image mcr.microsoft.com/dotnet/core/aspnet:3.1-alpine, and deploy docker-compose to portainer.</br>

docker-compose in step
```yaml
- name: deploy to portainer
  uses: LGinC/portainer-stack-deploy@master
  with: 
    serverurl: http://xxx.com:9000
    username: ${{ secrets.PORTAINER_USERNAME }}
    password: ${{ secrets.PORTAINER_PASSWORD }}
    endpointId: 1
    stackname: dotnet_test
    imagenames: |
        xxx/xxx
        myhub.com/xx1/xxx
    variables: |
        tag=6.0-alpine
    env: |
        TZ=Asia/Shanghai
        myTag=App
    docker_compose: |
      version: "3"
      services:
        dotnet_test:
          image:  mcr.microsoft.com/dotnet/core/aspnet:{{ tag }}
          container_name: dotnet_runtime       
```

or

<br>docker-compose in repository

```yaml
- name: portainer
  uses: LGinC/portainer-stack-deploy@master
  with:
    serverurl: http://xxxxx:9000
    access_token:  ${{ secrets.PORTAINER_AK }}
    endpointId: 2
    stackname: xxxservice
    imagenames: |
        xxx/xxx
        myhub.com/xx1/xxx
    env: |
        myTag:App
    docker_compose_path: deploy/docker-compose.yaml
    repo_username:  ${{ secrets.REPO_USERNAME }}
    repo_password:  ${{ secrets.REPO_PASSWORD }}
```
