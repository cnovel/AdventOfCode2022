use std::{fs, collections::HashSet};

fn can_move(pos: &mut(i32, i32), blocked_space: &HashSet<(i32,i32)>, abyss: i32) -> bool {
    if pos.1 + 1 == abyss {
        return false;
    }

    if !blocked_space.contains(&(pos.0, pos.1 + 1)) {
        pos.1 += 1;
        return true;
    }
    if !blocked_space.contains(&(pos.0 - 1, pos.1 + 1)) {
        pos.0 -= 1;
        pos.1 += 1;
        return true;
    }
    if !blocked_space.contains(&(pos.0 + 1, pos.1 + 1)) {
        pos.0 += 1;
        pos.1 += 1;
        return true;
    }
    return false;
}

fn sand_not_in_abyss(blocked_space: &mut HashSet<(i32, i32)>, abyss: i32) -> bool {
    let mut pos = (500, 0);
    while can_move(&mut pos, blocked_space, abyss) {
        if pos.1 == abyss - 1 {
            return false;
        }
    }
    blocked_space.insert(pos);
    return true;
}

fn sand_not_at_start(blocked_space: &mut HashSet<(i32, i32)>, abyss: i32) -> bool {
    let mut pos = (500, 0);
    while can_move(&mut pos, blocked_space, abyss) {}
    blocked_space.insert(pos);
    if pos.0 == 500 && pos.1 == 0 {
        return false;
    }
    return true;
}

fn solve(p: &str, s: &str) {
    println!("Solving Day 14 - {}", s);

    let contents = fs::read_to_string(&p).expect("Should have been able to read the file");
    let mut blocked_space: HashSet<(i32, i32)> = HashSet::new();

    let mut abyss = 0;

    for l in contents.split("\n").map(|l| l.trim()) {
        let c= l.split(" -> ").collect::<Vec<&str>>();
        let mut coords: Vec<(i32,i32)> = Vec::new();
        for i in &c {
            let s: Vec<i32> = i.split(",").collect::<Vec<&str>>().iter().map(|e| e.parse::<i32>().expect(e)).collect();
            coords.push((s[0], s[1]));
        }

        for i in 0..(coords.len() - 1) {
            let min_x = std::cmp::min(coords[i].0, coords[i+1].0);
            let max_x = std::cmp::max(coords[i].0, coords[i+1].0) + 1;
            let min_y = std::cmp::min(coords[i].1, coords[i+1].1);
            let max_y = std::cmp::max(coords[i].1, coords[i+1].1) + 1;
            if max_y > abyss {
                abyss = max_y;
            }
            for x in min_x..max_x {
                for y in min_y..max_y {
                    blocked_space.insert((x,y));
                }
            }
        }
    }
    abyss += 1;

    let mut score_1 = 0;
    let mut score_2 = 0;
    while sand_not_in_abyss(&mut blocked_space, abyss) {
        score_1 += 1;
        score_2 += 1;
    }
    println!("Score 1 = {}", score_1);
    while sand_not_at_start(&mut blocked_space, abyss) {
        score_2 += 1;
    }
    println!("Score 2 = {}", score_2 + 1);
}

pub fn solve_all() {
    solve("inputs/day_14_example_1.txt", "Example");
    solve("inputs/day_14.txt", "Main");
}