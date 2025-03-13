#Stage 1: Build the application

FROM rust:1.81 as builder
WORKDIR /usr/src/depclean
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

#Stage 2 : Create a minimal runtime image
FROM debian:buster-slim
COPY --from=builder /usr/src/depclean/target/release/depclean /usr/local/bin/depclean
ENTRYPOINT ["depclean"]
CMD ["--help"]