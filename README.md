<h1 align="center">theoludwig/advent_of_code_2023</h1>

<p align="center">
  <strong>My Solutions for the <a href="https://adventofcode.com/2023">Advent of Code 2023</a>, implemented in the <a href="https://www.rust-lang.org/">Rust Programming Language</a>.</strong>
</p>

<p align="center">
  <a href="./CONTRIBUTING.md"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat" alt="CONTRIBUTING" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/licence-MIT-blue.svg" alt="Licence MIT"/></a>
  <a href="./CODE_OF_CONDUCT.md"><img src="https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg" alt="Contributor Covenant" /></a>
  <br />
  <a href="https://github.com/theoludwig/advent_of_code_2023/actions/workflows/ci.yml"><img src="https://github.com/theoludwig/advent_of_code_2023/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI" /></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust%20MSRV-v1.74.0-blue?logo=rust" alt="Rust" /></a>
  <a href="https://conventionalcommits.org"><img src="https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg" alt="Conventional Commits" /></a>
</p>

## Getting Started

### Prerequisites

#### Rust Toolchain

The current Minimum Supported [Rust](https://www.rust-lang.org/) Version (MSRV) is **1.74.0**.

The project is **tested** against the following Rust versions:

- **Minimum Supported Rust Version (MSRV): v1.74.0**
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

## 💡 Contributing

Anyone can help to improve the project, submit a Feature Request, a bug report or even correct a simple spelling mistake.

The steps to contribute can be found in the [CONTRIBUTING.md](./CONTRIBUTING.md) file.

## 📄 License

[MIT](./LICENSE)
