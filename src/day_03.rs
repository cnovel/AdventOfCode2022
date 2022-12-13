use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn solve(p: &str, s: &str) {
    println!("Solving Day 03 - {}", s);

    let contents = fs::read_to_string(&p)
        .expect("Should have been able to read the file");

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut score_map: HashMap<char, i32> = HashMap::new();
    let mut i = 1;
    for c in alphabet.chars() {
        score_map.insert(c, i);
        i += 1;
    }

    let mut score = 0;
    let lines: Vec<&str> = contents.split("\n").map(|l| l.trim()).collect();
    for l in lines.iter() {
        let part_1 = &l[0..l.len()/2];
        let part_2 = &l[l.len()/2..l.len() - 1];
        let duplicate: HashSet<char> = part_1.chars().filter(|&c| part_2.contains(c)).collect();
        for c in duplicate.iter() {
            score += score_map[c];
        }
    }
    println!("Score 1 = {}", score);

    score = 0;
    for i in 0..lines.len()/3 {
        let pack = [lines[3*i], lines[3*i+1], lines[3*i+2]];
        let duplicate: HashSet<char> = pack[0].chars().filter(|&c| pack[1].contains(c)).filter(|&c| pack[2].contains(c)).collect();
        for c in duplicate.iter() {
            score += score_map[c];
        }
    }
    println!("Score 2 = {}", score);
}

pub fn solve_all() {
    solve("inputs/day_03_example_1.txt", "Example");
    solve("inputs/day_03.txt", "Main");
}