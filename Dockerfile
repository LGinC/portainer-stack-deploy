FROM alpine

COPY "entrypoint.sh" "/entrypoint.sh"
RUN sed -i "s@dl-cdn.alpinelinux.org@mirrors.aliyun.com@g" /etc/apk/repositories && apk --no-cache add curl jq && \
chmod +x entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]