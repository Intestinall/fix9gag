# Heavily inspired from https://docs.docker.com/guides/rust/develop/
ARG RUST_VERSION=1.91.1
ARG APP_NAME=fix9gag
FROM rust:${RUST_VERSION} AS build
ARG APP_NAME
WORKDIR /app

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF

FROM debian:trixie-slim AS final
RUN apt-get update && apt-get install -y ca-certificates
ARG UID=10001
RUN useradd \
    --no-create-home \
    --home-dir /nonexistent \
    --shell /sbin/nologin \
    --uid "${UID}" \
    --comment "" \
    appuser
USER appuser
COPY --from=build /bin/server /bin/

ENV FIX9GAG_HOST=0.0.0.0

CMD ["/bin/server"]
