# Use the official Rust image for building
ARG RUST_VERSION=1.75
FROM rust:${RUST_VERSION}-bookworm as builder

WORKDIR /app

# Copy the entire project and build it
COPY . .
RUN cargo build --release

# Stage 2: Create the final lightweight image
# TODO: User docker image with chrome testing browser
FROM debian:bookworm-slim

WORKDIR /app

# Copy the built executable from the previous stage
COPY --from=builder /app/target/release/squirrel /app
# Copy the sample workflow (for example purpose)
COPY src/sample_workflow.yaml /app

# Set the entry point to the executable
ENTRYPOINT ["./squirrel"]

# Set default parameters to entrypoint
CMD ["sample_workflow.yaml", "false"]
