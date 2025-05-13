FROM ubuntu:24.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libudev-dev \
    llvm \
    libclang-dev \
    protobuf-compiler \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust with SBF target
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup component add rustfmt
RUN rustup target add sbf-solana-solana

# Install Solana CLI with retries
RUN sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)" || \
    sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

# Install Anchor with SBF support
RUN cargo install --git https://github.com/coral-xyz/anchor avm --force \
    && avm install 0.31.1 \
    && avm use 0.31.1
RUN cargo install cargo-build-bpf

# Install Node.js and Yarn
RUN curl -sL https://deb.nodesource.com/setup_20.x | bash -
RUN apt-get install -y nodejs
RUN npm install -g yarn

# Configure environment
WORKDIR /workspace
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"
ENV PATH="/root/.cargo/bin:${PATH}"

# Keep container running
CMD ["bash"] 