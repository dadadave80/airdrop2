# airdrop2

A small Rust crate demonstrating common Solana Devnet workflows using the Solana Rust SDK. The logic lives in `src/lib.rs` and is exposed via test functions so you can run tasks with `cargo test -- --nocapture`.

Implemented tasks include:
- Keypair generation and key encoding helpers
- Requesting an airdrop on Devnet
- Transferring SOL
- Enrolling/minting via a specific on-chain program (Turbin3 prerequisite program)

The code targets Solana Devnet by default.

## Project structure
- `Cargo.toml` – crate metadata and dependencies
- `src/lib.rs` – all task implementations under `#[cfg(test)]`
- `dev-wallet.json` – local keypair file used for airdrop/transfer (not committed by default)
- `Turbin3-wallet.json` – keypair used for `enroll` (not committed by default)

## Requirements
- Rust toolchain (stable)
- Solana tool suite (for creating/inspecting keypairs): https://docs.solanalabs.com/
- A Devnet RPC endpoint (defaults to `https://api.devnet.solana.com` inside tests)

Dependencies (see `Cargo.toml`):
- `solana-client = 3.0.3`
- `solana-sdk = 3.0.0`
- `solana-program = 3.0.0`
- `solana-system-interface = 2.0.0`
- `bs58 = 0.5.1`

## Wallet files and security
- `dev-wallet.json` and `Turbin3-wallet.json` must contain a raw keypair as a JSON array of 64 bytes (the usual `solana-keygen` JSON format).
- These are sensitive secrets. Do not commit them. They are in `.gitignore`.

Use the test helper to print a new keypair you can copy to a file:
```bash
cargo test keygen -- --nocapture
```

## Running tasks
All tasks are exposed as Rust tests in `src/lib.rs`. Use `-- --nocapture` to see printed output.

- Keypair generation helper:
```bash
cargo test keygen -- --nocapture
```

- Convert Base58 private key string -> JSON array format:
```bash
cargo test base58_to_wallet -- --nocapture
# Paste your Base58-encoded private key when prompted
```

- Convert JSON array private key -> Base58 string:
```bash
cargo test wallet_to_base58 -- --nocapture
# Paste your JSON array (e.g. [12,34,...]) when prompted
```

- Claim 2 SOL airdrop on Devnet to `dev-wallet.json` (`claim_airdrop()` in `src/lib.rs`):
```bash
cargo test claim_airdrop -- --nocapture
```

- Transfer all available SOL (minus fee) from `dev-wallet.json` to a target address (`transfer_sol()` in `src/lib.rs`):
  1) Edit `src/lib.rs` and replace `"<my Turbin3 public key>"` with your destination public key.
  2) Run:
```bash
cargo test transfer_sol -- --nocapture
```

- Enroll/mint via the Turbin3 prerequisite program (`enroll()` in `src/lib.rs`):
  - Uses fixed program IDs and collection from the example:
    - Program: `TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM`
    - Collection: `5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2`
    - MPL Core Program: `CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d`
  - Signs with `Turbin3-wallet.json` and a fresh mint keypair.
```bash
cargo test enroll -- --nocapture
```
On success, the test prints a Solana Explorer URL for the confirmed transaction.

## RPC configuration
The tests use:
```rust
const RPC_URL: &str = "https://api.devnet.solana.com";
```
If you need a custom endpoint, change it in `src/lib.rs` tests module.

## Troubleshooting
- Ensure your wallet files exist and are valid JSON arrays of bytes.
- For airdrops, Devnet faucets may rate-limit; try again later.
- If `transfer_sol` fails with insufficient funds, first run `claim_airdrop`.
- Replace any placeholder public keys in `src/lib.rs` before running.

## Notes
- Code is organized as tests for convenience; you can refactor into a binary if desired.
- Keep your private keys secure. Never share or commit them.
