FROM rustlang/rust:nightly-slim

WORKDIR /app
COPY ./ /app

RUN set -ex; \ 
apt-get update; \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev

    RUN cargo build --release

# match nightly slim
    FROM debian:buster-slim
    WORKDIR /app
    COPY --from=0 /app/target/release/html_diff.
    RUN set -ex; \ 
    apt-get update; \
      apt-get install -y --no-install-recommends \
      libssl-dev
      CMD ["/app/html_diff"]
