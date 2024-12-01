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
        _ => todo!(),
    }
}
