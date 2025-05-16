Solana Development Environment Setup for zkid_backend_v1
This README provides commands to set up a Docker-based Solana development environment for the zkid_backend_v1 project on Ubuntu 22.04, resolving GLIBC mismatches, SBF toolchain issues, and network errors.
Prerequisites

Host: Ubuntu 22.04 with Docker installed (docker --version should show 27.3.1 or higher).
Disk Space: ~10GB free (df -h).
Memory: Sufficient RAM or swap (free -h).
Project: zkid_backend_v1 in /home/dracian/Projects/backend-v1/zkid_backend_v1.

Setup Instructions
1. Verify Docker
Ensure Docker is running and accessible without sudo:
```bash
docker --version
docker ps
```
If permission errors occur:
```bash
sudo usermod -aG docker $USER
newgrp docker
sudo systemctl start docker
sudo systemctl enable docker
```
2. Create/Verify Dockerfile
```bash
Create ~/Dockerfile:
nano ~/Dockerfile
```
Paste:
```
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

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Solana CLI
RUN sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

# Install Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor avm --force
RUN avm install 0.31.1
RUN avm use 0.31.1

# Install Node.js and Yarn
RUN curl -sL https://deb.nodesource.com/setup_23.x | bash -
RUN apt-get install -y nodejs
RUN npm install -g yarn
```
# Set working directory
```docker
WORKDIR /workspace
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"
```
# Keep container running
CMD ["bash"]
Save (Ctrl+O, Enter, Ctrl+X).
Verify:
```bash
cat ~/Dockerfile
```


3. Build Docker Image
Build the solana-dev image:
```bash
cd ~
docker build -t solana-dev .
```

Takes 10–20 minutes.
If image exists, remove first:docker rmi solana-dev



Verify:
docker images

Look for solana-dev.
4. Run Docker Container
Start the container with your project mounted:
```bash
docker run -it -v /home/dracian/Projects:/workspace/Projects solana-dev
```
5. Install Solana CLI and SBF Toolchain (Inside Container)
Install Solana CLI 2.2.12 with SBF toolchain from GitHub:
rm -rf /root/.local/share/solana
mkdir -p /root/.local/share/solana/install/releases/v2.2.12
curl -L -o /tmp/solana-release.tar.bz2 https://github.com/anza-xyz/agave/releases/download/v2.2.12/solana-release-x86_64-unknown-linux-gnu.tar.bz2
tar -xjf /tmp/solana-release.tar.bz2 -C /root/.local/share/solana/install/releases/v2.2.12/
solana-install init v2.2.12
echo 'export PATH="/root/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

6. Verify Tools
Check installed tools:
```docker
rustc --version
solana --version
anchor --version
node -v
yarn --version
```
Expected:
rustc 1.86.0 (05f9846f8 2025-03-31)
solana-cli 2.2.12 (src:0315eb6a; feat:1522022101, client:Agave)
anchor-cli 0.31.1
v23.x.x
1.22.22

7. Verify SBF Toolchain
Ensure SBF toolchain is present:
```docker
ls -la /root/.local/share/solana/install/releases/v2.2.12/solana-release/bin/sdk/sbf/dependencies/platform-tools/rust/lib
```
Expect files like librustc_std.a.
8. Fix Cargo.toml Warnings
Edit program’s Cargo.toml:
nano /workspace/Projects/backend-v1/zkid_backend_v1/programs/zkid_backend_v1/Cargo.toml
```text
Ensure:
[package]
name = "zkid_backend_v1"
version = "0.1.0"
description = "Created with Anchor"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]
name = "zkid_backend_v1"

[features]
default = []
cpi = ["no-entrypoint"]
no-entrypoint = []
no-idl = []
no-log-ix-name = []
idl-build = ["anchor-lang/idl-build"]

[dependencies]
anchor-lang = "0.31.1"
```
Check root Cargo.toml:
```docker
nano /workspace/Projects/backend-v1/zkid_backend_v1/Cargo.toml
```
Remove cargo-features = ["profile-overrides"]. Typical:
[workspace]
members = ["programs/zkid_backend_v1"]

9. Build Project
Build zkid_backend_v1:
```docker
cd /workspace/Projects/backend-v1/zkid_backend_v1
cargo update
anchor clean
anchor build
```
10. Exit and Reuse
Exit:
```docker
exit
```
Restart:
```bash
docker run -it -v /home/dracian/Projects:/workspace/Projects solana-dev
```
Troubleshooting

Docker Permission Errors:sudo systemctl start docker
sudo systemctl enable docker
docker ps


SBF Toolchain Missing:Reinstall Solana CLI:rm -rf /root/.local/share/solana
curl -L -o /tmp/solana-release.tar.bz2 https://github.com/anza-xyz/agave/releases/download/v2.2.11/solana-release-x86_64-unknown-linux-gnu.tar.bz2
tar -xjf /tmp/solana-release.tar.bz2 -C /root/.local/share/solana/install/releases/v2.2.11/
solana-install init v2.2.11
source ~/.bashrc


Build Fails:Share:anchor build --verbose
grep solana_program Cargo.lock


Network Issues:Use host networking:docker run -it --network host -v /home/dracian/Projects:/workspace/Projects solana-dev


Resources:Check: df -h, free -h.Add swap:sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile



Additional Notes

Solana Wallet:solana-keygen new --no-passphrase
solana config set --url devnet
solana airdrop 2
cp /root/.config/solana/id.json /workspace/Projects/solana-keypair.json


Local Testing:solana-test-validator

In another container:docker run -it -v /home/dracian/Projects:/workspace/Projects solana-dev
solana config set -ul
cd /workspace/Projects/backend-v1/zkid_backend_v1
anchor test



Contact
For issues, provide:

Container: solana --version, ls -la /root/.local/share/solana/install/releases/v2.2.12/solana-release/bin/sdk/sbf/dependencies/platform-tools/rust/lib, anchor build --verbose.
Host: docker images, free -h, lsb_release -a.

