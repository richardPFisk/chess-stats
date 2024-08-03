# Use the official Rust image as a parent image
FROM rust:latest AS builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code

COPY openings ./openings
COPY pgn ./pgn
COPY ui ./ui
COPY history ./history

# Build the application
RUN cargo build --release
RUN cargo install dioxus-cli

# # Use a smaller base image for the runtime
# FROM debian:buster-slim

# # Set the working directory in the container
# WORKDIR /usr/local/bin

# # Copy the built executable from the builder stage
# COPY --from=builder /usr/src/app/target/release/ .

# # Expose the port the app runs on
# EXPOSE 8080

# Run the binary
# CMD ["./your_app_name"]