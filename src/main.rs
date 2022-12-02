use std::env;

pub mod day_01;

fn main() {
    println!("Welcome to Advent of Code 2022");
    let args: Vec<String> = env::args().collect();
    let mut day = 0;
    if args.len() == 2 {
        println!("Running only day {}", args[1]);
        day = args[1].trim().parse().expect("Please type a number!");
    }
    else {
        println!("Running all days");
    }

    if day == 0 || day == 1 {
        day_01::solve_all();
    }
}
