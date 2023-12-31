# Referenes:
# https://hub.docker.com/_/debian
# https://www.debian.org/distrib/packages
# https://hub.docker.com/_/rust

# Use the official Rust image for building
ARG RUST_VERSION=1.75
FROM rust:${RUST_VERSION}-bullseye as builder

WORKDIR /app

# Copy files required to build executable
COPY ./src ./src
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY rust-toolchain ./

# Build the executable
RUN cargo build -v --release

# Stage 2: Create the final lightweight image
FROM debian:bullseye

WORKDIR /app

# Copy the built executable from the previous stage
COPY --from=builder /app/target/release/squirrel /app
# Copy the sample workflow (for example purpose)
COPY ./src/sample_workflow.yaml /app/src/

# Set the entry point to the executable
ENTRYPOINT ["./squirrel"]

# Set default arguments to entrypoint
CMD ["./src/sample_workflow.yaml", "http://host.docker.internal:9515", "true"]