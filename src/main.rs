use std::env;
use std::time;

pub mod day_01;
pub mod day_02;
pub mod day_03;

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

    let days = [day_01::solve_all, day_03::solve_all, day_03::solve_all];

    if day == 0 {
        let mut d = 1;
        for f in days.iter() {
            let now = time::Instant::now();
            f();
            let t = now.elapsed().as_secs_f64();
            println!("Day {} solved in {:.3}s", d, t);
            times.push(t);
            println!("----------");
            d += 1;
        }
    } else {
        let now = time::Instant::now();
        days[day - 1]();
        let t = now.elapsed().as_secs_f64();
        println!("Day {} solved in {:.3}s", day, t);
        times.push(t);
        println!("----------");
    }
    
    println!("Took {:.3}s for {} days [{:.3}s avg]", 
        times.iter().sum::<f64>(),
        times.len(),
        times.iter().sum::<f64>() / times.len() as f64);
}
