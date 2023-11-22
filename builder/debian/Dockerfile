FROM rust:slim-buster as builder

RUN USER=root cargo new --bin aircampi_auth_api
WORKDIR ./aircampi_auth_api
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml
#RUN cargo build --release
#RUN rm src/*.rs

ADD . ./

RUN apt-get install pkg-config libssl-dev \
    && cargo build --release


FROM debian:buster-slim

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /home/rust/src/aircampi_auth_api/target/release/aircampi_auth_api ${APP}/aircampi_auth_api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./aircampi_auth_api"]
