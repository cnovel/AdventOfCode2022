use std::{fs, collections::{HashSet, HashMap}};

fn find_shortest_v(dist: &Vec<i32>, vertices_to_view: &mut HashSet<i32>) -> i32 {
    let mut shortest = std::i32::MAX;
    let mut v_id = -1;
    for v in vertices_to_view.iter() {
        if dist[*v as usize] < shortest {
            shortest = dist[*v as usize];
            v_id = *v;
        }
    }

    vertices_to_view.remove(&v_id);
    return v_id;
}

fn get_neighbors_id(v_id: i32, w: i32, h: i32) -> Vec<i32> {
    let mut n = Vec::new();
    if v_id - w > 0 { n.push(v_id - w); }
    if v_id + w < w*h { n.push(v_id + w); }
    if v_id % w > 0 { n.push(v_id - 1); }
    if v_id % w < w - 1 { n.push(v_id + 1); }
    return n;
}

fn is_a(lines: &Vec<&str>, v_id: i32) -> bool {
    let i = v_id % lines[0].len() as i32;
    let j = v_id / lines[0].len() as i32;
    return lines[j as usize].chars().nth(i as usize).unwrap() == 'a';
}

fn solve(p: &str, s: &str) {
    println!("Solving Day 12 - {}", s);
    let contents = fs::read_to_string(&p).expect("Should have been able to read the file");

    let mut edges: HashSet<(i32, i32)> = HashSet::new();
    let mut vertices_to_view: HashSet<i32> = HashSet::new();
    let mut elevation_table: HashMap<char, i32> = HashMap::new();
    let mut h = 0;
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        elevation_table.insert(c, h);
        h += 1;
    }
    elevation_table.insert('S', 0);
    elevation_table.insert('E', 25);

    let mut start = 0;

    let lines: Vec<&str> = contents.split("\n").map(|l| l.trim()).collect();
    let nb_nodes = lines.len() * lines[0].len();
    let mut dist = vec![10000000; nb_nodes];
    let mut prev: Vec<i32> = vec![-1; nb_nodes];

    for j in 0..lines.len() {
        for i in lines[j].char_indices() {
            let e = i.1;
            let coord = (i.0 + j*lines[0].len()) as i32;
            vertices_to_view.insert(coord);
            if i.1 == 'S' {
                start = coord;
            }
            if i.1 == 'E' {
                dist[coord as usize] = 0;
            }

            if j > 0 && elevation_table[&lines[j-1].chars().nth(i.0).unwrap()] >= elevation_table[&e] - 1 {
                edges.insert((coord, (i.0 + (j-1)*lines[0].len()) as i32));
            }
            if i.0 > 0 && elevation_table[&lines[j].chars().nth(i.0 - 1).unwrap()] >= elevation_table[&e] - 1 {
                edges.insert((coord, (i.0 - 1 + j*lines[0].len()) as i32));
            }

            if i.0 + 1 < lines[0].len() && elevation_table[&lines[j].chars().nth(i.0 + 1).unwrap()] >= elevation_table[&e] - 1 {
                edges.insert((coord, (i.0 + 1 + j*lines[0].len()) as i32));
            }

            if j + 1 < lines.len() && elevation_table[&lines[j+1].chars().nth(i.0).unwrap()] >= elevation_table[&e] - 1 {
                edges.insert((coord, (i.0 + (j+1)*lines[0].len()) as i32));
            }
        }
    }

    let mut best_a = std::i32::MAX;
    while !vertices_to_view.is_empty() {
        let v_id = find_shortest_v(&dist, &mut vertices_to_view);
        if v_id == start {
            println!("Score 1 = {}", dist[v_id as usize]);
        }
        if is_a(&lines, v_id) && dist[v_id as usize] < best_a {
            best_a = dist[v_id as usize];
        }
        let mut neighs = get_neighbors_id(v_id, lines[0].len() as i32, lines.len() as i32);
        neighs.retain(|x| vertices_to_view.contains(x) && edges.contains(&(v_id, *x)));
        for n_id in neighs {
            let new_dist = dist[v_id as usize] + 1;
            if new_dist < dist[n_id as usize] {
                dist[n_id as usize] = new_dist;
                prev[n_id as usize] = v_id;
            }
        }
    }
    println!("Score 2 = {}", best_a);
}

pub fn solve_all() {
    solve("inputs/day_12_example_1.txt", "Example");
    solve("inputs/day_12.txt", "Main");
}