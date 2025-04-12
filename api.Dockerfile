# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Build the project to cache dependencies
RUN cargo build --release

# Remove the source code, to reduce image size
RUN rm -rf .

# Copy the rest of the source code
COPY . .

# Build the project
RUN cargo build --release

# Define the command to run your application
CMD ["cargo", "run", "--release"]