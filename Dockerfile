FROM rust:1.80-slim AS build
WORKDIR /usr/src

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl
# Download musl-tools for zstd
RUN apt-get update && apt-get -y install musl-tools

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new tfcsite-rs
WORKDIR /usr/src/tfcsite-rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=build /usr/local/cargo/bin/tfcsite-rs .
ADD ./static ./static
USER 1000
EXPOSE 3000
CMD ["./tfcsite-rs"]