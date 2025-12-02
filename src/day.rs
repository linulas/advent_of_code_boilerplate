use std::fmt::Display;

pub trait Solution<T: Display, F: Display> {
    fn part_one(&mut self) -> T;
    fn part_two(&mut self) -> F;
}

pub fn print_day<T: Display, F: Display>(input_day: u8, mut solution: impl Solution<T, F>) {
    println!(
        "Day {}:\n\tPart 1: {}\n\tPart 2: {}",
        input_day,
        solution.part_one(),
        solution.part_two()
    );
}

