FROM alpine
ARG TARGETARCH
ARG TARGETVARIANT
ADD drone-portainer-$TARGETARCH$TARGETVARIANT /usr/bin/drone-portainer
ENTRYPOINT ["/usr/bin/drone-portainer"]