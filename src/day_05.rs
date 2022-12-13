use std::fs;
use std::collections::VecDeque;

fn process(l: &str, stacks: &mut Vec<VecDeque<char>>) {
    let s: Vec<&str> = l.split(" ").collect();
    let iter: usize = s[1].parse().expect("Should be a number");
    let start: usize = s[3].parse::<usize>().expect("Should be a number") - 1;
    let end: usize = s[5].parse::<usize>().expect("Should be a number") - 1;

    for _i in 0..iter {
        let c = stacks[start].pop_front();
        if let Some(c) = c {
            stacks[end].push_front(c);
        }
    }
}

fn process_2(l: &str, stacks: &mut Vec<VecDeque<char>>) {
    let s: Vec<&str> = l.split(" ").collect();
    let iter: usize = s[1].parse().expect("Should be a number");
    let start: usize = s[3].parse::<usize>().expect("Should be a number") - 1;
    let end: usize = s[5].parse::<usize>().expect("Should be a number") - 1;

    let mut to_move: VecDeque<char> = VecDeque::new();
    for _i in 0..iter {
        let c = stacks[start].pop_front();
        if let Some(c) = c {
            to_move.push_back(c);
        }
    }
    while !to_move.is_empty() {
        stacks[end].push_front(to_move.pop_back().unwrap());
    }
}

fn get_word(stacks: &Vec<VecDeque<char>>) -> String {
    let mut word = String::from("");
    for s in stacks.iter() {
        let j = s.front();
        if let Some(j) = j {
            word.push(*j);
        }
    }
    return word;
}

fn fill_stacks(lines: &Vec<&str>, stacks: &mut Vec<VecDeque<char>>) {
    for l in lines {
        let mut c = 0;
        for x in (1..l.len()).step_by(4) {
            let s = l.chars().nth(x).unwrap();
            if s != ' ' {
                stacks[c].push_back(s);
            }
            c += 1;
        }
    }
}

fn solve(p: &str, s: &str) {
    println!("Solving Day 05 - {}", s);

    let contents = fs::read_to_string(&p)
        .expect("Should have been able to read the file");

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut stacks_2: Vec<VecDeque<char>> = Vec::new();
    let mut in_crates_lines: Vec<&str> = Vec::new();
    for l in contents.split("\r\n") {
        if l.contains('[') {
            in_crates_lines.push(l);
            continue;
        }

        if l.starts_with(" 1") {
            let s: Vec<&str> = l.trim().split(" ").collect();
            let nb_stacks: usize = s.last().unwrap().parse().expect("Should be a number");
            stacks = vec![VecDeque::new(); nb_stacks];
            println!("We have {} stacks", nb_stacks);
            continue;
        }

        if l.trim().is_empty() {
            fill_stacks(&in_crates_lines, &mut stacks);
            stacks_2 = stacks.clone();
            continue;
        }

        if l.starts_with("move") {
            process(l, &mut stacks);
            process_2(l, &mut stacks_2);
        }
    }

    let w = get_word(&stacks);
    println!("Word 1 is {}", w);
    let w2 = get_word(&stacks_2);
    println!("Word 2 is {}", w2);
}

pub fn solve_all() {
    solve("inputs/day_05_example_1.txt", "Example");
    solve("inputs/day_05.txt", "Main");
}