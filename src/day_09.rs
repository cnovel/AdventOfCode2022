use std::{fs, collections::HashSet};

fn get_new_t_pos(h_pos: &(i32,i32), t_pos: &(i32,i32)) -> (i32,i32) {
    let max_dist = std::cmp::max((h_pos.0 - t_pos.0).abs(), (h_pos.1 - t_pos.1).abs());
    if max_dist < 2 {
        return (t_pos.0, t_pos.1);
    }

    let mut v = (h_pos.0 - t_pos.0, h_pos.1 - t_pos.1);
    if v.0.abs() == 2 {
        v.0 /= 2;
    }
    if v.1.abs() == 2 {
        v.1 /= 2;
    }

    return (t_pos.0 + v.0, t_pos.1 + v.1);
}

fn solve(p: &str, s: &str) {
    println!("Solving Day 09 - {}", s);

    let contents = fs::read_to_string(&p)
        .expect("Should have been able to read the file");

    let mut rope_1 = vec![(0,0); 2];
    let mut rope_2 = vec![(0,0); 10];

    let mut all_t_pos_1: HashSet<(i32,i32)> = HashSet::new();
    all_t_pos_1.insert((0,0));
    let mut all_t_pos_2: HashSet<(i32,i32)> = HashSet::new();
    all_t_pos_2.insert((0,0));

    for l in contents.split("\n").map(|l| l.trim()) {
        let cmds: Vec<&str> = l.split(" ").collect();
        let moves: i32 = cmds[1].parse().expect("Should be a number");
        for _ in 0..moves {
            if cmds[0] == "U" {
                rope_1[0].1 += 1;
                rope_2[0].1 += 1;
            }
            if cmds[0] == "D" {
                rope_1[0].1 -= 1;
                rope_2[0].1 -= 1;
            }
            if cmds[0] == "L" {
                rope_1[0].0 -= 1;
                rope_2[0].0 -= 1;
            }
            if cmds[0] == "R" {
                rope_1[0].0 += 1;
                rope_2[0].0 += 1;
            }

            for i in 1..rope_1.len() {
                rope_1[i] = get_new_t_pos(&rope_1[i - 1], &rope_1[i]);
            }
            for i in 1..rope_2.len() {
                rope_2[i] = get_new_t_pos(&rope_2[i - 1], &rope_2[i]);
            }
            all_t_pos_1.insert(*rope_1.last().unwrap());
            all_t_pos_2.insert(*rope_2.last().unwrap());
        }
    }

    println!("Score 1: {}", all_t_pos_1.len());
    println!("Score 2: {}", all_t_pos_2.len());
}

pub fn solve_all() {
    solve("inputs/day_09_example_1.txt", "Example 1");
    solve("inputs/day_09_example_2.txt", "Example 2");
    solve("inputs/day_09.txt", "Main");
}