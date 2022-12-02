use std::fs;
use std::collections::HashMap;

fn compute_line_score_part_1(l: &str) -> i32 {
    let mut s;
    let c = l.chars().nth(2).expect("We should have 3 characters");
    if c == 'X' {
        s = 1;
    } else if c == 'Y' {
        s = 2;
    } else {
        s = 3;
    }

    let e = l.chars().nth(0).expect("We should have 3 characters");
    if e == 'A' && c == 'X' {
        s += 3;
    }
    if e == 'A' && c == 'Y' {
        s += 6;
    }
    if e == 'B' && c == 'Y' {
        s += 3;
    }
    if e == 'B' && c == 'Z' {
        s += 6;
    }
    if e == 'C' && c == 'Z' {
        s += 3;
    }
    if e == 'C' && c == 'X' {
        s += 6;
    }

    return s;
}

fn solve(p: &str, s: &str) {    
    println!("Solving Day 02 - {}", s);
    let mut score_1 = 0;
    let mut score_2 = 0;

    let mut scores_2_map = HashMap::new();
    scores_2_map.insert(String::from("A X"), 3 + 0);
    scores_2_map.insert(String::from("A Y"), 1 + 3);
    scores_2_map.insert(String::from("A Z"), 2 + 6);
    scores_2_map.insert(String::from("B X"), 1 + 0);
    scores_2_map.insert(String::from("B Y"), 2 + 3);
    scores_2_map.insert(String::from("B Z"), 3 + 6);
    scores_2_map.insert(String::from("C X"), 2 + 0);
    scores_2_map.insert(String::from("C Y"), 3 + 3);
    scores_2_map.insert(String::from("C Z"), 1 + 6);

    let contents = fs::read_to_string(&p)
        .expect("Should have been able to read the file");
    for l in contents.split("\n") {
        score_1 += compute_line_score_part_1(l.trim());
        score_2 += scores_2_map[l.trim()];
    }

    println!("Score 1: {}", score_1);
    println!("Score 2: {}", score_2);
}

pub fn solve_all() {
    solve("inputs/day_02_example_1.txt", "Example");
    solve("inputs/day_02.txt", "Main");
}