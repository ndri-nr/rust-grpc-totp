# ============================================================
# Stage 1: Build the Rust binary
# ============================================================
FROM rust:1-slim-bookworm AS builder

# Install protobuf compiler (required by tonic-build)
RUN apt-get update && apt-get install -y --no-install-recommends protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy dependency manifests first (leverages Docker layer caching)
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./
COPY proto ./proto

# Create a dummy main.rs to pre-build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY src ./src

# Build the real binary (only recompiles our code, deps are cached)
RUN touch src/main.rs && cargo build --release

# ============================================================
# Stage 2: Minimal runtime image
# ============================================================
FROM debian:bookworm-slim AS runtime

# Create a non-root user for security
RUN groupadd -r totp && useradd -r -g totp -s /sbin/nologin totp

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/totp /app/totp

# Set ownership to non-root user
RUN chown -R totp:totp /app

USER totp

# Default environment variables for container/Kubernetes usage
# Bind to 0.0.0.0 so the service is reachable from outside the container
ENV OTP_HOST=0.0.0.0
ENV OTP_PORT=50051

# Expose the gRPC port
EXPOSE 50051

# Health check (optional, useful for Kubernetes liveness probes)
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD ["/bin/sh", "-c", "echo 'healthy'"]

ENTRYPOINT ["/app/totp"]
