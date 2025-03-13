#Stage 1: Build the application

FROM rust:1.81 as builder
WORKDIR /usr/src/depclean
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

#Stage 2 : Create a minimal runtime image
FROM debian:bookworm-slim  # Bookworm has OpenSSL 3.x
COPY ./depclean /usr/local/bin/depclean
RUN chmod +x /usr/local/bin/depclean  # Ensure executable
CMD ["depclean"]