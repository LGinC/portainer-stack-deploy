FROM alpine

COPY "entrypoint.sh" "/entrypoint.sh"
RUN apk --no-cache add curl jq bash
ENTRYPOINT ["bash", "/entrypoint.sh"]