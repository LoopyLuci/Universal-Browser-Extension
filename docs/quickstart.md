# DNEA Quickstart

Full happy path on a clean machine.

## 1. Build

```bash
git clone https://github.com/dnea-project/dnea
cd dnea
cargo build --release
cargo install cargo-component --version 0.20.0 --locked
cargo component build -p focus-mode --release