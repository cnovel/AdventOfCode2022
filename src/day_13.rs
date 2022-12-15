use std::fs;
use json::{self, JsonValue};

fn compare_array(a_0: &JsonValue, a_1: &JsonValue) -> i32 {
    if a_0.is_empty() && !a_1.is_empty() {
        return 1;
    }
    if !a_0.is_empty() && a_1.is_empty() {
        return -1;
    }
    if a_0.is_empty() && a_1.is_empty() {
        return 0;
    }

    if a_0.is_number() && a_1.is_array() {
        return compare_array(&json::array![a_0.as_i32().unwrap()], a_1);
    }

    if a_0.is_array() && a_1.is_number() {
        return compare_array(a_0, &json::array![a_1.as_i32().unwrap()]);
    }

    if a_0.is_array() && a_1.is_array() {
        for i in 0..std::cmp::min(a_0.len(), a_1.len()) {
            let r = compare_array(&a_0[i], &a_1[i]);
            if r != 0 {
                return r;
            }
        }

        if a_0.len() < a_1.len() {
            return 1;
        }
        if a_0.len() > a_1.len() {
            return -1;
        }
        return 0;
    }

    if a_0.is_number() && a_1.is_number() {
        if a_0.as_i32().unwrap() < a_1.as_i32().unwrap() {
            return 1;
        }
        if a_0.as_i32().unwrap() > a_1.as_i32().unwrap() {
            return -1;
        }
        return 0;
    }

    return 0;
}

fn in_order(p: &(&str, &str, i32)) -> bool {
    let j0 = json::parse(p.0).expect("Should be valid");
    let j1 = json::parse(p.1).expect("Should be valid");

    return compare_array(&j0, &j1) == 1;
}

fn sort_fn(a: &str, b: &str) -> std::cmp::Ordering {
    if in_order(&(a,b,0)) {
        return std::cmp::Ordering::Less;
    }
    return std::cmp::Ordering::Greater;
}

fn solve(p: &str, s: &str) {
    println!("Solving Day 13 - {}", s);

    let contents = fs::read_to_string(&p).expect("Should have been able to read the file");
    let mut pairs: Vec<(&str, &str, i32)> = Vec::new();
    let mut pair: (&str, &str, i32) = ("","", 0);
    for l in contents.split("\n").map(|l| l.trim()) {
        if l.is_empty() {
            pair.2 = pairs.len() as i32 + 1;
            pairs.push(pair);
            pair.0 = "";
            pair.1 = "";
            continue;
        }
        if pair.0.is_empty() {
            pair.0 = l;
            continue;
        }
        pair.1 = l;
    }

    println!("Score 1 = {}", pairs.iter().map(|p| if in_order(p) { p.2 } else { 0 }).sum::<i32>());

    let mut orders: Vec<&str> = Vec::new();
    for l in contents.split("\n").map(|l| l.trim()) {
        if l.is_empty() {
            continue;
        }
        orders.push(l);
    }
    orders.push("[[2]]");
    orders.push("[[6]]");
    orders.sort_by(|a,b| sort_fn(a, b));
    let p = orders.iter().position(|r| r == &"[[2]]").unwrap() + 1;
    let q = orders.iter().position(|r| r == &"[[6]]").unwrap() + 1;
    println!("Score 2 = {}", p*q);
}

pub fn solve_all() {
    solve("inputs/day_13_example_1.txt", "Example");
    solve("inputs/day_13.txt", "Main");
}