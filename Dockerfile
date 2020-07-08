FROM alpine

RUN sed -i "s@dl-cdn.alpinelinux.org@mirrors.aliyun.com@g" /etc/apk/repositories && apk --no-cache add curl jq
COPY entrypoint.sh entrypoint.sh
ENTRYPOINT ["entrypoint.sh"]