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

The current Minimum Supported [Rust](https://www.rust-lang.org/) Version (MSRV) is **v1.74.0**.

The project is **tested** against the following Rust versions:

- **Minimum Supported Rust Version (MSRV): v1.74.0**
- **Latest Stable Version**

#### External Linting Tools (optional)

**External linting tools** are used to ensure a consistent code style and commit message format. They are **used in the Continuous Integration (CI)** pipeline and **can be optionally used locally**.

They have to be installed using [Node.js](https://nodejs.org/) >= v20.0.0 and [npm](https://www.npmjs.com/) >= v10.0.0.

- [editorconfig-checker](https://editorconfig-checker.github.io/) (`npm install --global editorconfig-checker@5.1.2`)
- [Prettier](https://prettier.io/) v3.2.4 (`npm install --global prettier@3.2.4`)
- [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2) v0.12.1 (`npm install --global markdownlint-cli2@0.12.1`)
- [commitlint](https://commitlint.js.org/#/) v18.6.0 (`npm install --global @commitlint/cli@18.6.0 @commitlint/config-conventional@18.6.0`)

### Usage

```sh
# Run a specific day's challenge (e.g. Day 1)
cargo run --package day_1

# Build, Lint, and Test Usage
cargo build
cargo test
cargo clippy --verbose -- -D warnings
cargo fmt -- --check

# External Linting Tools Usage (optional)
editorconfig-checker
prettier . --check
markdownlint-cli2
echo 'chore: try commitlint' | commitlint
```

## 💡 Contributing

Anyone can help to improve the project, submit a Feature Request, a bug report or even correct a simple spelling mistake.

The steps to contribute can be found in the [CONTRIBUTING.md](./CONTRIBUTING.md) file.

## 📄 License

[MIT](./LICENSE)
