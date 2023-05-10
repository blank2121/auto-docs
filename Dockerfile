# Use the official Ubuntu 22.04 LTS image
FROM ubuntu:22.04

# Update the package list and install dependencies
RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set the working directory to /app
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to /app
COPY Cargo.toml Cargo.lock ./

# Copy the entire project directory to /app
COPY . .

# Build the project using Cargo
RUN cargo build

# Start the application when the container is run
CMD ["cargo", "build", "--release"]
