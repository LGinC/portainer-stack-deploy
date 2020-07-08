FROM alpine

RUN sed -i "s@dl-cdn.alpinelinux.org@mirrors.aliyun.com@g" /etc/apk/repositories && apk --no-cache add curl jq

ENTRYPOINT ["entrypoint.sh"]