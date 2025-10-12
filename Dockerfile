# Multi-stage build for minimal final image
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY benches ./benches

# Build release binary
RUN cargo build --release

# Runtime image - minimal Debian
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/fabric-atelier /usr/local/bin/fabric-atelier

# Copy configuration
COPY config ./config

# Copy patterns (git submodule)
COPY data ./data

# Add MCP server name annotation (REQUIRED for registry)
LABEL io.modelcontextprotocol.server.name="io.github.copyleftdev/fabric-atelier"
LABEL org.opencontainers.image.title="Fabric Atelier"
LABEL org.opencontainers.image.description="AI-powered content processing with 226 Fabric patterns via MCP"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.authors="copyleftdev"
LABEL org.opencontainers.image.url="https://github.com/copyleftdev/fabric-atelier"
LABEL org.opencontainers.image.source="https://github.com/copyleftdev/fabric-atelier"
LABEL org.opencontainers.image.licenses="MIT"

# Set environment defaults
ENV RUST_LOG=info
ENV LLM_MODEL=llama3

# Expose stdio transport (MCP standard)
ENTRYPOINT ["fabric-atelier"]
