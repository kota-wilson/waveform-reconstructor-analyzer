# Development Environment

## Project Root

`/Users/kota/Desktop/softwareai/projects/ferrisoxide`

## Isolation Level

Level 1: project folder plus Cargo workspace.

## Tooling

- Rust toolchain.
- Cargo.
- rustfmt.
- clippy.

## Dependency Policy

- Approved MVP crates are `csv`, `serde`, `serde_json`, and `toml`; see `docs/dependency-review.md`.
- Any additional third-party crate requires dependency review, license review, and explicit approval.
- Do not install global tools for this project without approval.

## Expected Commands

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
