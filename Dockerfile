FROM rust as builder
WORKDIR /ya-vm-file-server
RUN rustup default nightly
RUN rustup default stable
COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src
COPY tests ./tests
RUN cargo build --release --bin ya-vm-file-server --features="build-binary"
RUN cargo +nightly test --release --no-run

FROM debian:stable-slim
WORKDIR /work
COPY --from=builder /ya-vm-file-server/target/release .
RUN mkdir ./server_root_fs
ENTRYPOINT ["./ya-vm-file-server", "--mount-point", "server_root_fs"]


