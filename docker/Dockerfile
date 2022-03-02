FROM alpine
ARG TARGETARCH
ARG TARGETVARIANT
ADD portainer-deploy-$TARGETARCH$TARGETVARIANT /usr/bin/portainer-deploy
ENTRYPOINT ["/usr/bin/portainer-deploy"]