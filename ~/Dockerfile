FROM ubuntu:22.04

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

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Solana CLI with direct download
RUN mkdir -p /root/.local/share/solana/install/releases \
    && curl -L -o /tmp/solana.tar.bz2 https://github.com/anza-xyz/agave/releases/download/v1.18.15/solana-release-x86_64-unknown-linux-gnu.tar.bz2 \
    && tar -xjf /tmp/solana.tar.bz2 -C /root/.local/share/solana/install/releases/ \
    && echo 'export PATH="/root/.local/share/solana/install/releases/solana-release/bin:$PATH"' >> /root/.bashrc

ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Then install SBF tools
RUN cargo install --git https://github.com/solana-labs/solana cargo-run-sbf
RUN cargo install cargo-build-bpf

# Install Anchor with SBF support
RUN cargo install --git https://github.com/coral-xyz/anchor avm --force \
    && avm install 0.31.1 \
    && avm use 0.31.1

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