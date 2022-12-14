use std::env;
use std::time;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;

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

    let days = [day_01::solve_all, day_02::solve_all, day_03::solve_all, day_04::solve_all,
        day_05::solve_all, day_06::solve_all, day_07::solve_all, day_08::solve_all, day_09::solve_all,
        day_10::solve_all, day_11::solve_all, day_12::solve_all];

    let mut d = 0;
    for f in days.iter() {
        d += 1;
        if day != 0 && d != day {
            continue;
        }
        let now = time::Instant::now();
        f();
        let t = now.elapsed().as_secs_f64();
        println!("Day {} solved in {:.3}s", d, t);
        times.push(t);
        println!("----------");
    }

    println!("Took {:.3}s for {} days [{:.3}s avg]",
        times.iter().sum::<f64>(),
        times.len(),
        times.iter().sum::<f64>() / times.len() as f64);
}
