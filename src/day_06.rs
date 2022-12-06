use std::{fs, collections::{VecDeque, HashSet}};

fn get_first_marker(line: &str, size: usize) -> i32{
    let mut last_n : VecDeque<char> = VecDeque::new();
    let mut marker = 0;
    for c in line.chars() {
        if last_n.len() == size {
            last_n.pop_front();
        }
        last_n.push_back(c);
        marker += 1;
        if last_n.len() == size {
            let s: HashSet<char> = last_n.clone().into_iter().collect();
            if s.len() == size {
                return marker;
            }
        }
    }
    return -1;
}

fn solve(p: &str, s: &str) {
    println!("Solving Day 06 - {}", s);
    
    let contents = fs::read_to_string(&p)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").map(|l| l.trim()).collect();
    for l in lines {
        println!("First marker is {}", get_first_marker(&l, 4));
        println!("Second marker is {}", get_first_marker(&l, 14));
    }
}

pub fn solve_all() {
    solve("inputs/day_06_examples.txt", "Example");
    solve("inputs/day_06.txt", "Main");
}