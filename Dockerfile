# Stage 1 - Install cargo-chef
FROM rust:alpine AS chef
# Install the required dependencies (since we are going to statically link the binary)
# curl is required here because the `gen-api-docs` binary uses a lib that includes a fetch 
# of Swagger UI during the build process
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconf curl

# `SYSROOT` is defaulted to `/usr`, but pkg-config-rs will always dynamically link
# to the libraries found there. Since we want to statically link all dependencies,
# we need to set `SYSROOT` to a dummy directory.
ENV SYSROOT=/dummy

# Install cargo-chef
RUN cargo install cargo-chef
WORKDIR /build

# Stage 2 - Build recipe for dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3 - Cache dependencies
FROM chef AS cacher
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 4 - Build the API documentation
FROM cacher AS doc_builder
COPY . .
# Copy the cached dependencies
COPY --from=cacher /build/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
# Generate the API documentation
RUN mkdir ./api-doc
RUN cargo run --bin gen-api-docs

# Stage 5 - Build the final executable
FROM doc_builder AS app_builder
COPY . .
# Copy the cached dependencies
COPY --from=cacher /build/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin app

# final base
FROM scratch AS runtime
COPY --from=doc_builder /build/api-doc /api-doc
COPY --from=app_builder /build/target/release/app /app
EXPOSE 1337

CMD ["/app"]