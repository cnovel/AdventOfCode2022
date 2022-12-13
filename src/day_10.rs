use std::fs;

fn value_if_strength(cycle: &i32, x: &i32) -> i32 {
    if [20, 60, 100, 140, 180, 220].iter().any(|i| i == cycle) {
        //println!("At {}th : x = {} -> {}", cycle, x, x*cycle);
        return *x * cycle;
    }
    return 0;
}

fn update_row(crt_row: &mut String, x: &i32) {
    if crt_row.len() as i32 > x - 2 && (crt_row.len() as i32) < (x + 2) {
        crt_row.push('#');
    }
    else {
        crt_row.push(' ');
    }
    if crt_row.len() == 40 {
        println!("{}", crt_row);
        crt_row.clear();
    }
}

fn solve(p: &str, s: &str) {
    println!("Solving Day 10 - {}", s);

    let contents = fs::read_to_string(&p)
        .expect("Should have been able to read the file");

    let mut x = 1;
    let mut cycle = 1;
    let mut score_1 = 0;
    let mut crt_row = "".to_string();
    for l in contents.split("\n").map(|l| l.trim()) {
        if l.starts_with("noop") {
            update_row(&mut crt_row, &x);
            cycle += 1;
            score_1 += value_if_strength(&cycle, &x);
            continue;
        }
        // Start cycle
        update_row(&mut crt_row, &x);
        let d: i32 = l.split(" ").last().unwrap().parse().expect("Should be a number");
        cycle += 1;
        update_row(&mut crt_row, &x);
        score_1 += value_if_strength(&cycle, &x);
        x += d;
        cycle += 1;
        score_1 += value_if_strength(&cycle, &x);
    }
    println!("Score = {}", score_1);
}

pub fn solve_all() {
    solve("inputs/day_10_example_1.txt", "Example");
    solve("inputs/day_10.txt", "Main");
}