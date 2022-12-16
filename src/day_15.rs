use std::{fs, collections::HashSet};
use regex::Regex;

fn solve(p: &str, s: &str, y: i32) {
    println!("Solving Day 15 - {}", s);

    let contents = fs::read_to_string(&p).expect("Should have been able to read the file");
    let mut sensors: Vec<(i32, i32)> = Vec::new();
    let mut beacons: Vec<(i32, i32)> = Vec::new();
    let mut man_dist: Vec<i32> = Vec::new();
    let re = Regex::new(r"-?[0-9]+").unwrap();
    for l in contents.split("\n").map(|l| l.trim()) {
        let mut m: Vec<i32> = Vec::new();
        for cap in re.captures_iter(l) {
            m.push(cap[0].parse().expect("Should be a number"));
        }
        sensors.push((m[0], m[1]));
        beacons.push((m[2], m[3]));
        man_dist.push((m[0] - m[2]).abs() + (m[1] - m[3]).abs());
    }

    let mut impossible_beacons: HashSet<(i32, i32)> = HashSet::new();
    for i in 0..sensors.len() {
        let s = sensors[i];
        let d = man_dist[i];
        if (s.1 - y).abs() > d {
            continue;
        }
        let diff = d - (s.1 - y).abs();
        for x in (s.0 - diff)..(s.0 + diff +1) {
            let pos = (x, y);
            if !beacons.contains(&pos) {
                impossible_beacons.insert(pos);
            }
        }
    }

    println!("Score 1 = {}", impossible_beacons.len());
}

pub fn solve_all() {
    solve("inputs/day_15_example_1.txt", "Example", 10);
    solve("inputs/day_15.txt", "Main", 2000000);
}