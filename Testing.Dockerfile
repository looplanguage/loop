# This Dockerfile is used as the testing environment for Picasso
FROM rust:1.58

# Copy files needed to build dependencies
WORKDIR /app
COPY src/dummy.rs src/dummy.rs
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

# Hacky magic to only build dependencies
RUN sed -i 's/lib.rs/dummy.rs/' Cargo.toml
RUN cargo build
RUN sed -i 's/dummy.rs/lib.rs/' Cargo.toml

# Install last needed components
RUN rustup component add clippy
RUN rustup component add rustfmt
