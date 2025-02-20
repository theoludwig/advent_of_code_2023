<h1 align="center">theoludwig/advent_of_code</h1>

<p align="center">
  <strong>My Solutions for the <a href="https://adventofcode.com/">Advent of Code</a>, implemented in the <a href="https://www.rust-lang.org/">Rust Programming Language</a>.</strong>
</p>

## Days Progress

### [2024](./advent_2024)

- [Day 1 (`**`)](./advent_2024/puzzle_2024_day_1)
- [Day 2 (`**`)](./advent_2024/puzzle_2024_day_2)

### [2023](./advent_2023)

- [Day 1 (`**`)](./advent_2023/puzzle_2023_day_1)
- [Day 2 (`**`)](./advent_2023/puzzle_2023_day_2)
- [Day 3 (`**`)](./advent_2023/puzzle_2023_day_3)
- [Day 4 (`**`)](./advent_2023/puzzle_2023_day_4)
- [Day 5 (`**`)](./advent_2023/puzzle_2023_day_5)
- [Day 6 (`**`)](./advent_2023/puzzle_2023_day_6)
- [Day 7 (`*`)](./advent_2023/puzzle_2023_day_7)
- [Day 8 (`*`)](./advent_2023/puzzle_2023_day_8)

## Prerequisite: Rust Toolchain

The project is **tested** against the following [Rust](https://www.rust-lang.org/) versions:

- **Minimum Supported Rust Version (MSRV): v1.84.1**
- **Latest Stable Version**

## Usage

```sh
# Run a specific day's challenge (e.g. Day 1 of 2023)
cargo run --package puzzle_2023_day_1

# Create a new day's challenge (e.g. Day 1 of 2023)
cd advent_2023
cargo new puzzle_2023_day_1

# Build, Lint, and Test Usage
cargo build
cargo test
cargo clippy --verbose -- -D warnings
cargo fmt -- --check
```
