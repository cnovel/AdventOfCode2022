use std::fs;

fn solve(p: &str, s: &str) {
    println!("Solving Day 04 - {}", s);
    
    let contents = fs::read_to_string(&p)
        .expect("Should have been able to read the file");

    let mut score_1 = 0;
    let mut score_2 = 0;
    let lines: Vec<&str> = contents.split("\n").collect();
    for l in lines.iter() {
        let s: Vec<i32> = l.trim().split(|c| c == '-' || c == ',').map(|c| c.parse().expect("Should be a number")).collect();
        if (s[0] >= s[2] && s[1] <= s[3]) || (s[2] >= s[0] && s[3] <= s[1]) {
            score_1 += 1;
        }

        let max_left = std::cmp::max(s[0], s[2]);
        let min_right =  std::cmp::min(s[1], s[3]);
        if max_left <= min_right {
            score_2 += 1;
        }
    }

    println!("Self contained pairs = {}", score_1);
    println!("Overlapping pairs = {}", score_2);
}

pub fn solve_all() {
    solve("inputs/day_04_example_1.txt", "Example");
    solve("inputs/day_04.txt", "Main");
}