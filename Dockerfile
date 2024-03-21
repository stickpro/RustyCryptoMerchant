FROM rust:latest as builder

WORKDIR /workspace

RUN apt update && apt install lld clang -y

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt update && apt install -y --no-install-recommends openssl ca-certificates && apt clean

WORKDIR /workspace

COPY static static

COPY settings settings

# copy binary and configuration files
COPY --from=builder /workspace/target/release/app .

# expose port
EXPOSE 8080

ENV APP_PROFILE prod

ENV RUST_LOG info

# run the binary
ENTRYPOINT ["./app"]