# Conway's Sequence Generator

This program generates and analyzes Conway's Sequence in Rust.

## Description

Conway's Sequence is generated using the "look and say" algorithm starting from a seed value. Each term in the sequence describes the previous term.

This program allows generating Conway's Sequence terms, saving the sequence data, analyzing properties of the sequence, and visualizing the sequence in different ways.

## Features

- Generate Conway Sequence terms from a starting seed
- Apply Conway's "look and say" algorithm
- Save sequence data to file
- Analyze sequence properties:
  - Length of terms
  - Digits used
  - Frequency of digits
  - Growth rate

## Usage

The program can be run with:

```bash
cargo build
cargo run
```

You can customize the seed value and the number of iterations by providing appropriate command-line arguments. For example, to start the sequence from the seed value "11121" and generate 20 iterations:

```bash
cargo run -- --seed 11121 --iterations 10
cargo run -- -s 11121 -i 10
```

## References

- [More Info on Conway's Sequence](https://en.wikipedia.org/wiki/Look-and-say_sequence)

## Contributions

Contributions are welcome! Please open an issue or PR
