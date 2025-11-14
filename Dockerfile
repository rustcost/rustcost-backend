# syntax=docker/dockerfile:1

################################################################################
# Build stage
################################################################################
ARG RUST_VERSION=1.90.0
ARG APP_NAME=rustcost
FROM rust:${RUST_VERSION}-slim-bullseye AS build

ARG APP_NAME
WORKDIR /app

# Install minimal build dependencies (openssl optional, remove if not used)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Use build cache mounts for Cargo registry & target for faster rebuilds
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    /bin/sh -c "\
        set -e && \
        cargo build --locked --release && \
        mkdir -p /out && \
        cp /app/target/release/${APP_NAME} /out/${APP_NAME} \
    "

################################################################################
# Runtime stage
################################################################################
FROM debian:bullseye-slim AS final

ARG UID=10001
ARG APP_NAME=rustcost
WORKDIR /app

# Install minimal runtime dependencies (e.g., SSL certs if using HTTPS)
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
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

# Copy the compiled binary from the build stage
COPY --from=build /out/${APP_NAME} /usr/local/bin/${APP_NAME}

# Expose API port
EXPOSE 3000

# Default entrypoint
ENTRYPOINT ["/usr/local/bin/rustcost-core"]
CMD ["serve"]
