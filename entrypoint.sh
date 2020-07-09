#!/bin/bash

set -e

# set default endpointid=1
if [ -z "$INPUT_ENDPOINTID" ]; then
 $INPUT_ENDPOINTID=1
fi

#auth
Token_Result=$(curl --location --request POST ''${INPUT_SERVERURL}'/api/auth' \
--data-raw '{"Username":"'$INPUT_USERNAME'", "Password":"'$INPUT_PASSWORD'"}')
# Token_Result = {"jwt":"xxxxxxxx"}
#todo: get token failed  exit 1
token=$(echo $Token_Result | jq -r '.jwt')
#get stacks
stacks=$(curl --location --request GET ''${INPUT_SERVERURL}'/api/stacks' \
--header 'Authorization: Bearer '$token'')
length=$(echo $stacks | jq '.|length')
if [ $length > 0 ]; then
#find the stack name of INPUT_STACKNAME
  stackId=$(echo $stacks | jq '.[] | select(.Name=="'$INPUT_STACKNAME'") | .Id')
  if [ $stackId > 0 ]; then
 #find the stack id, and delete it
    echo 'delete stack id='$stackId''
    curl --location --request DELETE ''${INPUT_SERVERURL}'/api/stacks/'${stackId}'' --header 'Authorization: Bearer '$token''
  fi
fi

#create stacks

#compose=$(echo $INPUT_DOCKER_COMPOSE | sed 's/"/\"/')
echo 'docker-compose: '${INPUT_DOCKER_COMPOSE}''
result=$(curl POST ''${INPUT_SERVERURL}'/api/stacks?endpointId='$INPUT_ENDPOINTID'&method=string&type=2' \
-H 'Authorization: Bearer '$token'' \
-H 'Content-Type: application/json' \
--data-raw '{"Name":"'$INPUT_STACKNAME'","StackFileContent":"'${INPUT_DOCKER_COMPOSE}'"}")
echo $result
message=$(echo $result | jq -r '.message')
if [ -n "$message" ]; then
   echo 'create failed:'$message''
   exit 1
fi
