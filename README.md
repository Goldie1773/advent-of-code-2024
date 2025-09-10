# Advent of Code 2024

This repository contains my solutions for the Advent of Code 2024 challenges, implemented in Rust. Each day's challenge is organized into its own directory, allowing for clear separation of code and dependencies.

## Project Structure

- **Cargo.toml**: Main configuration file for the Rust project, specifying dependencies and metadata.
- **rust-toolchain.toml**: Specifies the Rust toolchain version for consistent development environments.
- **.gitignore**: Lists files and directories to be ignored by Git, such as build artifacts.
- **README.md**: Documentation for the project, including an overview and instructions.
- **aoc-common/**: Contains shared code used across different days.
  - **Cargo.toml**: Configuration for the common library.
  - **src/lib.rs**: Shared library code.
- **day-01/**: Contains the solution for the first day's challenge.
  - **Cargo.toml**: Configuration for day one.
  - **src/main.rs**: Entry point for day one.
- **day-02/**: Contains the solution for the second day's challenge.
  - **Cargo.toml**: Configuration for day two.
  - **src/main.rs**: Entry point for day two.
- **day-03/**: Contains the solution for the third day's challenge.
  - **Cargo.toml**: Configuration for day three.
  - **src/main.rs**: Entry point for day three.
- **scripts/**: Contains scripts for automating tasks.
  - **run_day.sh**: Script to run the code for a specific day.

## Getting Started

To get started with the project, clone the repository and navigate to the `advent-of-code-2024` directory:

```bash
git clone <repository-url>
cd advent-of-code-2024
```

### Running the Code

You can run the code for a specific day by navigating to the corresponding directory and using Cargo:

```bash
cd day-01
cargo run
```

### Contributing

Feel free to contribute by submitting pull requests or opening issues. Happy coding!