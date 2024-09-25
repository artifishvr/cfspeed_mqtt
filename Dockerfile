FROM rust:1.81 AS builder
WORKDIR /usr/src/cfspeed_mqtt
COPY . .
RUN cargo install --path .

FROM debian:12-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/* && ldconfig
COPY --from=builder /usr/local/cargo/bin/cfspeed_mqtt /usr/local/bin/cfspeed_mqtt
CMD ["cfspeed_mqtt"]
