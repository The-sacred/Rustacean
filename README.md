# Rustacean

Rustacean is a collection of small projects I build while learning Rust. Each project gives me a chance to practice Rust concepts, from basic input and control flow to ownership, borrowing, structs, and enums.

This repository will grow as I work through the language. The code is part of the learning process, so projects may be simple or change as I discover better ways to write them.

## Projects

| Project | Description | Concepts practiced |
| --- | --- | --- |
| [`guessing_game`](./guessing_game/) | A number guessing game. Enter guesses until you find the random number, or press `q` to quit. | User input, variables, loops, `match`, parsing, `Result`, and `Ordering` |
| [`score_grader`](./score_grader/) | Reads a score from 0 to 100 and prints its letter grade. | Functions, input, parsing, validation, conditionals, and return values |

## Run the projects

### Guessing game

From the repository root, run:

```sh
cargo run --manifest-path guessing_game/Cargo.toml
```

### Score grader

Compile the standalone Rust file:

```sh
rustc score_grader/main.rs -o score_grader
```

Then run the executable produced by `rustc`. Its filename and the command to run it depend on your operating system.

## Rust concepts

Concepts I am practicing across these projects include:

- Variables, mutability, and data types
- Functions and control flow
- Ownership and borrowing
- Structs and enums
- Pattern matching with `match`
- Error handling and input validation
- Collections, modules, traits, and generics

The list will evolve as I add projects and explore more of Rust.
