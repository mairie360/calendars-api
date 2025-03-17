# Define build arguments
ARG RUST_VERSION=1.85.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

# Set working directory
WORKDIR /usr/src/calendars-api

# Install dependencies for building
RUN apt-get update && apt-get install -y --no-install-recommends \
    binutils libpq-dev curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the source code
COPY . .

# Fetch dependencies
RUN cargo fetch --locked

# Build the application
RUN cargo build --release --locked && \
    strip target/release/calendars-api

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates wget libpq5 curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user and group
RUN groupadd --system calendars-api && useradd --no-log-init --system -g calendars-api calendars-api

# Copy the compiled binary
COPY --from=builder --chown=calendars-api:calendars-api /usr/src/calendars-api/target/release/calendars-api /usr/local/bin/calendars-api

# Set permissions
USER calendars-api

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/calendars-api"]
CMD []
