FROM debian:buster-slim
# Set environment variables
ENV RUSTUP_HOME="/opt/rust"
ENV CARGO_HOME="/opt/rust"
ENV PATH="$PATH:$RUSTUP_HOME/bin"
ENV RUST_VERSION=nightly-2022-05-14
# Install dependencies
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
        ca-certificates apt-transport-https gnupg \
        libssl-dev pkg-config \
        curl \
        git \
        software-properties-common \
        llvm clang make cmake && \
    rm -rf /var/lib/apt/lists/*
# Install docker
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add -
RUN add-apt-repository \
        "deb https://download.docker.com/linux/debian $(lsb_release -cs) stable" && \
    apt-get update && \
    apt-get install --no-install-recommends -y \
        docker-ce docker-ce-cli containerd.io && \
    rm -rf /var/lib/apt/lists/*
# Install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain ${RUST_VERSION} && \
    rustup default ${RUST_VERSION} && \
    rustup target add wasm32-unknown-unknown
# Build project
ADD . /app
WORKDIR /app
RUN cargo build --release && mv target / && rm -rf /app
WORKDIR /