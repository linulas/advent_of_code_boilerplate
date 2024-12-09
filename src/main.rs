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
        1 => todo!(),
        2 => todo!(),
        3 => todo!(),
        4 => todo!(),
        5 => todo!(),
        6 => todo!(),
        7 => todo!(),
        8 => todo!(),
        9 => todo!(),
        10 => todo!(),
        11 => todo!(),
        12 => todo!(),
        13 => todo!(),
        14 => todo!(),
        15 => todo!(),
        16 => todo!(),
        17 => todo!(),
        18 => todo!(),
        19 => todo!(),
        20 => todo!(),
        21 => todo!(),
        22 => todo!(),
        23 => todo!(),
        24 => todo!(),
        25 => todo!(),
        _ => unreachable!(),
    }
}
