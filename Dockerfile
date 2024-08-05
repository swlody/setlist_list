FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
ARG SQLX_OFFLINE=true
RUN cargo build --release --bin setlist_list

# Install bun deps (cached)
FROM oven/bun:1 AS base
WORKDIR /app/assets

FROM base AS install
RUN mkdir -p /temp/dev/assets
COPY assets/package.json assets/bun.lockb /temp/dev/assets
RUN bun install --cwd /temp/dev/assets --frozen-lockfile

# Copy install to workdir and generate static assets
FROM base as prerelease
WORKDIR /app/assets
COPY assets/ .
COPY --from=install /temp/dev/assets/node_modules node_modules
RUN bun tailwindcss  -c tailwind.config.js -i styles/input.css -o static/dist/output.css --minify
RUN bun build src/main.js --outdir static/dist --minify

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/setlist_list /usr/local/bin
COPY --from=prerelease /app/assets/static/ assets/static
COPY assets/views/ assets/views
RUN mkdir config
COPY config/production.yaml config

ENTRYPOINT ["/usr/local/bin/setlist_list"]
CMD ["start", "--environment", "production", "--server-and-worker"]
