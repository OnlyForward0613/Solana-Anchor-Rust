# Solana Anchor Rust

My first Rust program on Solana built with [Anchor](https://www.anchor-lang.com/)


## 1. Getting Started with Rust
https://doc.rust-lang.org/book/ch01-01-installation.html

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

# Install C compiler if on macOS
xcode-select --install

# Check that everything is installed correctly
rustup --version
rustc --version
cargo --version


```



## 2. Installing Solana 

https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

```bash
# Install Solana CLI tool (or replace version number with latest)
sh -c "$(curl -sSfL https://release.solana.com/v1.10.32/install)"
solana-install update

# Check that Solana is installed and PATH has been updated
# You might need to restart terminal for it to take effect
solana --version

# Run this if you want to develop on localhost first
solana config set --url localhost
solana config get

# Also get Mocha for your JavaScript testing later
npm install -g mocha

```


## 3. Installing Anchor

https://www.anchor-lang.com/docs/installation

```bash
# Create new default keypair for program tests
solana-keygen new

# Install yarn
npm install --global yarn

# Install Anchor version manager (avm)
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Check that everything works 
anchor --version

# You will also need Solana Web3 JS later
npm install @project-serum/anchor @solana/web3.js

```

## 4. Creating the Project
```bash
# You can always create a new project with this
anchor init myepicproject --javascript
cd myepicproject
anchor test

```

### Source Codes 
Refer to: [lib.rs](anchor-project/programs/anchor-project/src/lib.rs) 
and [anchor-project.js](anchor-project/tests/anchor-project.js).

--- 

# How to Deploy to Solana Devnet

```bash
# Make sure you're on devnet
solana config set --url devnet
solana config get

# If you ever run out of funds
solana airdrop 2

# Build the program
anchor build

# Get the new [program id]
solana address -k target/deploy/anchor_project.json

# Update Anchor.toml and lib.rs with new [program id]
# Make sure Anchor.toml is on devnet

# Build again
anchor build

# Deploy to devnet
solana program deploy target/deploy/anchor_project.so

# Test that everything is working
anchor test


```

## Deployed Address 🚀
https://explorer.solana.com/address/BcsgHu3sCQkC5DfMjbqM74zJEdtY4m3ZfDqMVjGYeWcU?cluster=devnet
