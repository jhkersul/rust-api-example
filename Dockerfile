FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/rust-api-example
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/rust-api-example /usr/local/bin/rust-api-example
CMD ["rust-api-example"]
