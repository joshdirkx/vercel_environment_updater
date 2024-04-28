FROM rust:1.77-bullseye as builder

WORKDIR /usr/src/vercel_environment_updater

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/vercel_environment_updater /usr/local/bin/vercel_environment_updater

RUN chmod +x /usr/local/bin/vercel_environment_updater

ENTRYPOINT ["/usr/local/bin/vercel_environment_updater"]