# Advent of Code 2023

## About

This repository contains my solutions for the [Advent of Code 2023](https://adventofcode.com/2023) challenges implemented in the [Rust Programming Language](https://www.rust-lang.org/).

## Getting Started

### Prerequisites

The current Minimum Supported [Rust](https://www.rust-lang.org/) Version (MSRV) is **1.70.0** (determined thanks to [cargo-msrv](https://crates.io/crates/cargo-msrv)).

The project is **tested** against the following Rust versions:
- **Minimum Supported Rust Version (MSRV): v1.70.0**
- **Latest Stable Version**

### Usage

```sh
# Run a specific day's challenge (e.g. Day 1)
cargo run --package day_1

# Build, Lint, and Test
cargo build
cargo test
cargo clippy --verbose -- -D warnings
cargo fmt -- --check
```

## License

[MIT](./LICENSE)
