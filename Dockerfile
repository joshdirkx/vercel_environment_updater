FROM rust:1.77 as builder

WORKDIR /usr/src/vercel_environment_updater

COPY . .

RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/vercel_environment_updater /usr/local/bin/vercel_environment_updater

RUN chmod +x ./vercel_environment_updater

CMD ["./vercel_environment_updater"]