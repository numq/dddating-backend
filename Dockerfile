FROM rust:alpine
ARG SERVICE_NAME
ENV SERVICE_NAME=$SERVICE_NAME
ENV RELEASE=1
WORKDIR /build
COPY library library/
COPY proto proto/
COPY .env .
COPY dependencies.toml .
WORKDIR /build/service/$SERVICE_NAME
COPY service/$SERVICE_NAME/src src/
COPY service/$SERVICE_NAME/build.rs .
COPY service/$SERVICE_NAME/Cargo.lock .
COPY service/$SERVICE_NAME/Cargo.toml .
RUN apk update && apk add musl-dev openssl-dev protobuf-dev
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
ARG SERVICE_NAME
ENV SERVICE_NAME=$SERVICE_NAME
COPY --from=0 /build/.env .
COPY --from=0 /build/service/$SERVICE_NAME/target/x86_64-unknown-linux-musl/release/$SERVICE_NAME .
CMD ["/bin/sh", "-c", "./${SERVICE_NAME}"]