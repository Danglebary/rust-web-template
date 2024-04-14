FROM rust:1.77-alpine as builder

# Install the required dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconf

# Set `SYSROOT` to a dummy path (default is /usr) because pkg-config-rs *always*
# links those located in that path dynamically but we want static linking, c.f.
# https://github.com/rust-lang/pkg-config-rs/blob/54325785816695df031cef3b26b6a9a203bbc01b/src/lib.rs#L613
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
