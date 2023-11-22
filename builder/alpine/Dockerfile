FROM ekidd/rust-musl-builder:stable as builder
#FROM rust:1.43 as builder

RUN USER=root cargo new --bin hischool_classroom_api
WORKDIR ./hischool_classroom_api
#COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml
#RUN cargo build --release
#RUN rm src/*.rs

ADD . ./

#RUN apk upgrade --update-cache --available && \
#    apk add openssl openssl-dev \
#    && apk add pkgconf perl \
#    && apk add curl cmake gcc g++ clang brotli musl postgresql \
#    && rm -rf /var/cache/apk/*
#RUN apk add openssl git curl cmake gcc g++ --no-cache pkgconfig postgresql clang brotli musl 

#RUN curl -O https://www.openssl.org/source/openssl-1.1.0f.tar.gz
#RUN tar xf openssl-1.1.0f.tar.gz
#RUN openssl-1.1.0f
#RUN export CC=...
#RUN ./Configure --prefix=... linux-x86_64 -fPIC
#RUN make -j$(nproc)
#RUN make install

#ENV OPENSSL_DIR=/usr/bin/openssl

RUN cargo build --release


FROM alpine:latest

ARG APP=/usr/src/app

EXPOSE 8536

ENV TZ=Etc/UTC \
    APP_USER=appuser \
    AUTH_DATABASE_URL=postgres://auth:Welcome1@localhost:5432/auth

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/hischool_classroom_api/target/x86_64-unknown-linux-musl/release/hischool_classroom_api ${APP}/hischool_classroom_api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./hischool_classroom_api"]
