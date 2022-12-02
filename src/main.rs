use std::env;
use std::time;

pub mod day_01;
pub mod day_02;

fn main() {
    println!("Welcome to Advent of Code 2022");
    let args: Vec<String> = env::args().collect();
    let mut times: Vec<f64> = Vec::new();
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
        let now = time::Instant::now();
        day_01::solve_all();
        let t = now.elapsed().as_secs_f64();
        println!("Day 1 solved in {:.3}s", t);
        times.push(t);
        println!("----------");
    }

    if day == 0 || day == 2 {
        let now = time::Instant::now();
        day_02::solve_all();
        let t = now.elapsed().as_secs_f64();
        println!("Day 2 solved in {:.3}s", t);
        times.push(t);
        println!("----------");
    }
}
