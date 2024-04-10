FROM rustlang/rust:nightly-buster-slim as builder

RUN apt update && apt install -y pkg-config libssl-dev
# RUN apt-get update && apt-get install && apt-get install pkg-config

# create a new empty shell project
RUN USER=root cargo new --bin rust-web-template
WORKDIR /rust-web-template

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy your source tree
COPY ./src ./src

# generate openapi spec
RUN mkdir ./api-doc
RUN cargo run --bin gen-openapi

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/api*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=builder /rust-web-template/target/release/api .
COPY --from=builder /rust-web-template/api-doc/openapi.json ./api-doc/openapi.json

EXPOSE 1337

CMD ["./api"]