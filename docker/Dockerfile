FROM alpine:latest

ARG TARGETARCH

WORKDIR /app
COPY ./build/cc-server-$TARGETARCH ./iris_backend

EXPOSE 8080
EXPOSE 8443

CMD [ "/app/iris_backend" ]
