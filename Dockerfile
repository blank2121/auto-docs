# Use Ubuntu 22.04 as the base image
FROM ubuntu:22.04

# Install necessary packages to install Rust and Cargo
RUN apt-get update \
    && apt-get install -y curl build-essential

# Install Rust and Cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set the environment variables for Rust
ENV PATH="/root/.cargo/bin:${PATH}"
ENV RUST_BACKTRACE="1"

# Set the working directory
WORKDIR /app

# Copy the application files to the working directory
COPY . .

# Build the application
RUN cargo build --release
