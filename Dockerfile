FROM rust:1.77-alpine as builder

# Install the required dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconf

# `SYSROOT` is defaulted to `/usr`, but pkg-config-rs will always dynamically link
# to the libraries found there. Since we want to statically link all dependencies,
# we need to set `SYSROOT` to a dummy directory.
ENV SYSROOT=/dummy

# Set the working directory
WORKDIR /build

# Copy the Cargo.toml and Cargo.lock files
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Copy the source code
COPY ./src ./src
COPY ./lib ./lib
COPY ./gen-api-docs ./gen-api-docs

# Generate the API documentation
RUN mkdir ./api-doc
RUN cargo run --bin gen-api-docs

# Run build to cache dependencies
RUN cargo build --bin app --release && rm ./src/*.rs

# Copy the source code again
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/app*
RUN cargo build --bin app --release

# final base
FROM scratch

# TODO: add labels

# Copy the built binary and API documentation
COPY --from=builder ./build/api-doc ./api-doc
COPY --from=builder ./build/target/release/app ./app

# Expose the port
EXPOSE 1337

# Run the binary
CMD ["./app"]
