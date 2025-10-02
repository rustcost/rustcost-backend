# syntax=docker/dockerfile:1

################################################################################
# Build stage
################################################################################
ARG RUST_VERSION=1.90.0
ARG APP_NAME=rustcost
FROM rust:${RUST_VERSION}-slim-bullseye AS build

ARG APP_NAME
WORKDIR /app

# Install dependencies for building with Postgres (Diesel/SQLx)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libpq-dev pkg-config libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Build the application
# Use build cache mounts for Cargo registry & target
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=migrations,target=migrations \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    /bin/sh -c "\
        set -e && \
        cargo build --locked --release && \
        mkdir -p /out && \
        cp /app/target/release/${APP_NAME} /out/rustcost \
    "

################################################################################
# Runtime stage
################################################################################
FROM debian:bullseye-slim AS final

ARG UID=10001
WORKDIR /app

# Install runtime dependencies for Postgres client libraries
RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Add non-root user
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the binary and migrations from builder
COPY --from=build /out/rustcost /usr/local/bin/rustcost

# Expose API port
EXPOSE 3000

# Default entrypoint (can be overridden by Helm chart for migrations)
ENTRYPOINT ["rustcost"]
CMD ["serve"]
