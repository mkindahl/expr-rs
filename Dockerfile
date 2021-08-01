# Create a statically-linked Rust application using Docker's
# multi-stage build. This is based on the blog post by Alexander Brand
# available at
# https://alexbrand.dev/post/how-to-package-rust-applications-into-minimal-docker-containers

# syntax=docker/dockerfile:1
FROM rust:1.52.0 AS builder
WORKDIR /usr/src/

# We are adding a MUSL target to allow building a statically linked
# Rust binary. For more information, see the blog post by Brenden Hyde
# available at https://bxbrenden.github.io/
RUN rustup target add x86_64-unknown-linux-musl

# Create a dummy crate for managing the build cache and avoid
# rebuilding all dependencies unless necessary.
#
# The binaries need to be in the right path so that we can build them,
# but they do not have to do anything sensible, so we create an
# application crate and move the main.rs file to the right path as
# given in the manifest.
#
# This will allow us to build and cache all dependencies and avoid
# re-building this layer unless the dependencies have changed.
RUN USER=root cargo new expr-rs
WORKDIR /usr/src/expr-rs
RUN mkdir -p src/bin && mv src/main.rs src/bin/expr.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# If all worked well, we copy the source and build and install the
# statically linked binaries. The binaries will be installed in the
# default location /usr/local/cargo/bin.
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/expr .
USER 1000
ENTRYPOINT ["./expr"]
