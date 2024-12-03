# Advent of Code boilerplate

This is a boilerplate for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

## Getting started

Get the boilerplate using [cargo generate](https://github.com/cargo-generate/cargo-generate):

```bash
# If you don't already have it, install cargo-generate
cargo install cargo-generate

# Generate the boilerplate
cargo generate --git https://github.com/linulas/advent_of_code_boilerplate

# Or optionally, just clone the repo (but if you want to use git,
# you'll need to delete the .git folder and initialize a new repo)
git clone https://github.com/linulas/advent_of_code_boilerplate
```

## Puzzle setup

1. Create a .env file and add:

```bash
# .env

# Get session cookie from the developer tools when visiting https://adventofcode.com
SESSION=your_session_cookie
YEAR=2024
```

2. Run the cli to import a specific day by running:

```shell
cargo run --bin aoc
```

3. Add the solution to `./src/main.rs`:

```rust
// ./src/main.rust

use self::solutions::day01::Day01; // import the solution
use crate::day::print_day;
use std::env;

#[cfg(test)]
mod test;

mod day;
mod solutions;

const ARGUMENT_ERROR: &str = "Please provide a day (number 1-24).";
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{ARGUMENT_ERROR}");
        return;
    }

    let number = match args[1].parse::<u8>() {
        Ok(num) => {
            if !(1..=25).contains(&num) {
                println!("{ARGUMENT_ERROR}");
                return;
            }
            num
        }
        Err(_) => {
            println!("{ARGUMENT_ERROR}");
            return;
        }
    };

    match number {
        1 => print_day(1, Day01::new(include_str!("./input/01.txt"))), // add the input
        _ => todo!(),
    }
}
```

### Manual setup (optional)
If you don't want to use the cli, you can manually setup the project as follows:

1. Create a rust file in `./src/solutions` (can be any name, e.g. `day01.rs`) and add it to `./src/solutions/mod.rs`:

```rust
// ./src/solutions/day01.rs

use crate::day::Solution;

pub struct Day01 { }

impl Day01 {
    fn new (input: 'static str) -> Self {
        todo!()
    }
}

// add implementations for printing the solutions to stdout
impl Solution<u64, u64> for Day01 {
    fn part_one(&mut self) -> u64 {
        todo!()
    }
    fn part_two(&mut self) -> u64 {
        todo!()
    }
}
```

```rust
//./src/solutions/mod.rs

pub mod day01;
```

2. Add the puzzle input to `./src/input/01.txt`:

```text
# ./src/input/01.txt

your puzzle input
```

3. Add the solution to `./src/main.rs`:

```rust
// ./src/main.rust

use self::solutions::day01::Day01; // import the solution
use crate::day::print_day;
use std::env;

#[cfg(test)]
mod test;

mod day;
mod solutions;

const ARGUMENT_ERROR: &str = "Please provide a day (number 1-24).";
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{ARGUMENT_ERROR}");
        return;
    }

    let number = match args[1].parse::<u8>() {
        Ok(num) => {
            if !(1..=25).contains(&num) {
                println!("{ARGUMENT_ERROR}");
                return;
            }
            num
        }
        Err(_) => {
            println!("{ARGUMENT_ERROR}");
            return;
        }
    };

    match number {
        1 => print_day(1, Day01::new(include_str!("./input/01.txt"))), // add the input
        _ => todo!(),
    }
}
```

## Testing

```bash
cargo test -- --nocapture --test day_one
```

## Running against the real input

```bash
cargo run 1
```
