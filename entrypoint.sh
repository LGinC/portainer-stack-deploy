#!/bin/bash

set -e

# set default endpointid=1
if [ -z "$INPUT_ENDPOINTID" ]; then
 $INPUT_ENDPOINTID=1
fi

stack=$(typeset -l $INPUT_STACKNAME) #ToLowerCase

#auth
echo
echo 'get token  : '${INPUT_SERVERURL}'/api/auth'
Token_Result=$(curl --location --request POST ''${INPUT_SERVERURL}'/api/auth' \
--data-raw '{"Username":"'$INPUT_USERNAME'", "Password":"'$INPUT_PASSWORD'"}')
# Token_Result = {"jwt":"xxxxxxxx"}
#todo: get token failed  exit 1
token=$(echo $Token_Result | jq -r '.jwt')
#get stacks
echo
echo 'get statcks :  '${INPUT_SERVERURL}'/api/stacks'
stacks=$(curl --location --request GET ''${INPUT_SERVERURL}'/api/stacks' \
--header 'Authorization: Bearer '$token'')
length=$(echo $stacks | jq '.|length')
if [ $length > 0 ]; then
#find the stack name of INPUT_STACKNAME
  stackId=$(echo $stacks | jq '.[] | select(.Name=="'$stack'") | .Id')
  if [ $stackId > 0 ]; then
 #find the stack id, and delete it
    echo
    echo 'delete stack id='$stackId'  '${INPUT_SERVERURL}'/api/stacks/'${stackId}' '
    curl --location --request DELETE ''${INPUT_SERVERURL}'/api/stacks/'${stackId}'' --header 'Authorization: Bearer '$token''
  fi
fi

#create stacks
compose=$(echo "$INPUT_DOCKER_COMPOSE" | sed 's#\"#\\"#g' | sed ":a;N;s/\\n/\\\\n/g;ta") # replace charactor  "->\"   \n -> \\n
echo
echo 'create stack  : '${INPUT_SERVERURL}'/api/stacks?endpointId='$INPUT_ENDPOINTID'&method=string&type=2'

echo
#echo "{\"Name\":\"'${INPUT_STACKNAME}'\",\"StackFileContent\":\"${compose}\",\"Env\":[]}"

result=$(curl --location --request POST ''${INPUT_SERVERURL}'/api/stacks?endpointId='$INPUT_ENDPOINTID'&method=string&type=2' \
--header 'Authorization: Bearer '${token}'' \
--header 'Content-Type: application/json' \
--data-raw "{\"Name\":\"'${stack}'\",\"StackFileContent\":\"${compose}\",\"Env\":[]}")
echo "$result"
echo
message=$(echo $result | jq -r '.message')
if [ $message != 'null' ]; then
  echo 'create failed:    '$message''
  exit 1
fi
exit 0

