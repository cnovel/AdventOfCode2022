use std::{fs, collections::HashMap};

fn solve(p: &str, s: &str) {
    println!("Solving Day 08 - {}", s);

    let contents = fs::read_to_string(&p)
        .expect("Should have been able to read the file");

    let mut trees: HashMap<(usize, usize), u32> = HashMap::new();

    let lines: Vec<&str> = contents.split('\n').map(|l| l.trim()).collect();
    for j in 0..lines.len() {
        let l = lines[j];
        for i in 0..l.len() {
            let h = l.chars().nth(i).unwrap().to_digit(10).unwrap();
            trees.insert((i,j), h);
        }
    }

    let width = lines[0].len();
    let height = lines.len();
    let mut score_1 = 0;
    let mut score_2 = 0;
    for j in 0..height {
        for i in 0..width {
            let h = trees[&(i,j)];
            let mut tree_dists: [u32; 4] = [0,0,0,0];
            let mut met_edge = false;
            for n in (0..j).rev() {
                tree_dists[0] += 1;
                if trees[&(i,n)] >= h {
                    break;
                }
                if n == 0 {
                    met_edge = true;
                }
            }
            for n in i+1..width {
                tree_dists[1] += 1;
                if trees[&(n,j)] >= h {
                    break;
                }
                if n == width - 1 {
                    met_edge = true;
                }
            }
            for n in j+1..height {
                tree_dists[2] += 1;
                if trees[&(i,n)] >= h {
                    break;
                }
                if n == height - 1 {
                    met_edge = true;
                }
            }
            for n in (0..i).rev() {
                tree_dists[3] += 1;
                if trees[&(n,j)] >= h {
                    break;
                }
                if n == 0 {
                    met_edge = true;
                }
            }

            if met_edge || i == 0 || i == width - 1 || j == 0 || j == height - 1 {
                score_1 += 1;
            }
            score_2 = std::cmp::max(score_2, tree_dists.into_iter().reduce(|a,b| a*b).unwrap());
        }
    }

    println!("Score 1 = {}", score_1);
    println!("Score 2 = {}", score_2);
}

pub fn solve_all() {
    solve("inputs/day_08_example_1.txt", "Example");
    solve("inputs/day_08.txt", "Main");
}