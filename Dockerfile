
FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/grpc-health-dummy
COPY . .

RUN cargo +nightly install grpc-health-dummy --path .

FROM debian:buster-slim

RUN apt-get update && \
		rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/grpc-health-dummy /usr/local/bin/grpc-health-dummy

EXPOSE 50051

CMD grpc-health-dummy
