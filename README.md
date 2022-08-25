# Solana Anchor Rust
My first Rust project on Solana built with Anchor


## How to Deploy
```
solana config set --url devnet

// Make sure you're on devnet.
solana config get

anchor build

// Get the new [program id].
solana address -k target/deploy/anchor_project.json

// Update Anchor.toml and lib.rs w/ new [program id].
// Make sure Anchor.toml is on devnet.

// Build again.
anchor build

// Deploy to devnet
solana program deploy target/deploy/anchor_project.so

```
