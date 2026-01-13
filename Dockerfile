# Multi-stage build for bunnypmsl
# trunk-ignore-all(hadolint/DL3008)
# Stage 1: Build the application
FROM rust:1.92.0-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install --no-install-recommends -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir -p src

# Copy source code
COPY src ./src

# Build the application (server only)
RUN cargo build --release --no-default-features --features server

# Stage 2: Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install --no-install-recommends -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder
COPY --from=builder /app/target/release/bunnypmsl /app/bunnypmsl

# Create bunnypmsl user and system config directory
RUN useradd -m -u 1000 bunnypmsl && \
    mkdir -p /etc/bunnypmsl

# Copy Docker-specific config template
COPY config.toml.docker /etc/bunnypmsl/config.toml

# Set ownership
RUN chown -R bunnypmsl:bunnypmsl /app /etc/bunnypmsl

USER bunnypmsl

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
# Config file at /etc/bunnypmsl/config.toml sets address to 0.0.0.0 for Docker
CMD ["/app/bunnypmsl", "serve"]
