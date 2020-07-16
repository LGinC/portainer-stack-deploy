# portainer-deploy
This action deploy image by portainer

# Inputs
---
## serverurl
**required**  portainer server url. like this: http://xxx.com:9000 <br>
portainer服务器公网地址  需要公网能够访问

## username
**required**  portainer username <br>
portainer 面板登录所用的用户名

## password
**required**  portainer password <br>
portainer 面板登录所用的密码

## endpointId
portainer endpoint id,default 1,  localhost is 1 <br>
portainer终结点id，默认是1,即第一个，一般为localhost

## stackname
**required** name of stack <br>
服务栈的名称，会在stacks列表里显示

## imagename
**required** name of pull image, like:  mcr.microsoft.com/dotnet/core/aspnet:3.1-alpine <br>
将会进行拉取镜像的镜像名

## docker_compose
**required** content of docker-compose.yml.  ps: portainer just support version: "2" <br>
docker-compose.yml的内容 注意:portainer目前仅支持version: "2" <br>
like this:
```
docker_compose: |
  version: "2"
  services:
  destinyapi:
    image:  mcr.microsoft.com/dotnet/core/aspnet:3.1-alpine
    container_name: dotnet_runtime
```
<br>
    
# Example usage
The following will pull image mcr.microsoft.com/dotnet/core/aspnet:3.1-alpine, and deploy docker-compose to portainer.
The following will delete it if same name stack is existed.
执行顺序为 如果有同名的stack会先删除此stack，然后拉取镜像，部署docker-compose内容到stack
```
- name: deploy to portainer
  uses: LGinC/portainer-stack-deploy@master
  with: 
    serverurl: http://xxx.com:9000
    username: ${{ secrets.PORTAINER_USERNAME }}
    password: ${{ secrets.PORTAINER_PASSWORD }}
    endpointId: 1
    stackname: dotnet_test
    imagename: mcr.microsoft.com/dotnet/core/aspnet:3.1-alpine
    docker_compose: |
      version: "2"
      services:
        destinyapi:
          image:  mcr.microsoft.com/dotnet/core/aspnet:3.1-alpine
          container_name: dotnet_runtime       
```
