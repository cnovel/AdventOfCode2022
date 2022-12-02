use std::env;

pub mod day_01;
pub mod day_02;

fn main() {
    println!("Welcome to Advent of Code 2022");
    let args: Vec<String> = env::args().collect();
    let mut day = 0;
    if args.len() == 2 {
        println!("Running only day {}", args[1]);
        day = args[1].trim().parse().expect("Please type a number!");    
        println!("----------");
    }
    else {
        println!("Running all days");
        println!("----------");
    }

    if day == 0 || day == 1 {
        day_01::solve_all();
        println!("----------");
    }

    if day == 0 || day == 2 {
        day_02::solve_all();
        println!("----------");
    }
}
