# syntax=docker/dockerfile:1

################################################################################
# Build stage
################################################################################
ARG RUST_VERSION=1.90.0
ARG APP_NAME=rustcost-core

FROM rust:${RUST_VERSION}-slim-bullseye AS build

ARG APP_NAME
WORKDIR /app

# Install build dependencies (OpenSSL optional)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config libssl-dev ca-certificates rsync && \
    rm -rf /var/lib/apt/lists/*

# Use Docker build cache for Cargo registry & target
RUN --mount=type=bind,source=.,target=/src \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    bash -c "\
        rsync -a /src/ . && \
        cargo build --release --locked && \
        mkdir -p /out && \
        cp target/release/${APP_NAME} /out/${APP_NAME} \
    "

################################################################################
# Runtime stage
################################################################################
FROM debian:bullseye-slim AS final

ARG UID=10001
ARG APP_NAME=rustcost-core
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/usr/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

RUN mkdir -p /app/data && \
    chown -R appuser:appuser /app

USER appuser

# Copy binary
COPY --from=build /out/${APP_NAME} /usr/local/bin/${APP_NAME}

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/rustcost-core"]
CMD ["serve"]
