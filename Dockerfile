FROM debian:stable

RUN apt-get update
RUN apt-get install -y curl build-essential socat sudo

# Add non-root user
ENV DOCK_USER=dock
ENV DOCK_USER_PSWD=dock
# RUN adduser --disabled-password --gecos '' $DOCK_USER
RUN useradd --create-home --shell /bin/bash ${DOCK_USER}
RUN echo "${DOCK_USER}:${DOCK_USER_PSWD}" | chpasswd
RUN adduser ${DOCK_USER} sudo
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# Non root operations
USER ${DOCK_USER}
RUN mkdir -p /home/dock/ya-vm-file-server
WORKDIR /home/dock/ya-vm-file-server

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/home/dock/.cargo/bin:${PATH}"
RUN rustup default nightly
RUN rustup default stable
COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src
COPY tests ./tests
RUN cargo build --release --bin ya-vm-file-server --features="build-binary"
RUN cargo +nightly test --release --no-run
RUN mkdir mnt_tests

RUN mkdir -p /home/dock/work
WORKDIR /home/dock/work

COPY docker_client_start.sh .
COPY docker_client_external_start.sh .
COPY docker_server_start.sh .
RUN mkdir ./server_root_fs
RUN mkdir ./client_fs
