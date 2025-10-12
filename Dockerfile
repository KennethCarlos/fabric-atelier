# syntax=docker/dockerfile:1

# ============================================================================
# Stage 1: Planner - Generate dependency recipe with cargo-chef
# ============================================================================
FROM rust:1.90 AS planner
WORKDIR /app
RUN cargo install cargo-chef --locked
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY benches ./benches
RUN cargo chef prepare --recipe-path recipe.json

# ============================================================================
# Stage 2: Builder - Cache dependencies and build binary
# ============================================================================
FROM rust:1.90 AS builder
WORKDIR /app

# Install cargo-chef for dependency caching
RUN cargo install cargo-chef --locked

# Copy dependency recipe from planner
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this layer is cached unless dependencies change
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# Copy source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY benches ./benches

# Build application - dependencies are already cached
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release --bin fabric-atelier && \
    cp /app/target/release/fabric-atelier /app/fabric-atelier

# ============================================================================
# Stage 3: Runtime - Minimal production image
# ============================================================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN groupadd -g 1001 appgroup && \
    useradd -u 1001 -g appgroup -m -d /home/appuser -s /bin/bash appuser

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder --chown=appuser:appgroup /app/fabric-atelier /usr/local/bin/fabric-atelier

# Copy patterns (git submodule)
COPY --chown=appuser:appgroup data ./data

# Add MCP server name annotation (REQUIRED for registry)
LABEL io.modelcontextprotocol.server.name="io.github.copyleftdev/fabric-atelier"
LABEL org.opencontainers.image.title="Fabric Atelier"
LABEL org.opencontainers.image.description="AI-powered content processing with 226 Fabric patterns via MCP"
LABEL org.opencontainers.image.version="0.1.0"
LABEL org.opencontainers.image.authors="copyleftdev"
LABEL org.opencontainers.image.url="https://github.com/copyleftdev/fabric-atelier"
LABEL org.opencontainers.image.source="https://github.com/copyleftdev/fabric-atelier"
LABEL org.opencontainers.image.licenses="MIT"

# Set environment defaults
ENV RUST_LOG=info \
    LLM_MODEL=llama3

# Switch to non-root user
USER appuser

# Expose stdio transport (MCP standard)
ENTRYPOINT ["/usr/local/bin/fabric-atelier"]
