use std::{fs, collections::{VecDeque, HashMap, HashSet}};

fn path_from_deque(p: &VecDeque<&str>) -> String {
    let mut path: String = String::from("");
    for e in p.iter() {
        path += &e.to_string();
        if *e != "/" {
            path += "/";
        }
    }
    return path;
}

fn path_for_file(p: &VecDeque<&str>, s: &str) -> String {
    return path_from_deque(p) + "/" + s;
}

fn solve(p: &str, s: &str) {
    println!("Solving Day 07 - {}", s);

    let contents = fs::read_to_string(&p)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").map(|l| l.trim()).collect();

    let mut cur_tree = VecDeque::<&str>::new();
    let mut all_dirs: HashSet<String> = HashSet::new();
    let mut all_dirs_size: HashMap<String, i32> = HashMap::new();
    let mut all_files_size: HashMap<String, i32> = HashMap::new();

    for l in lines {
        if l.starts_with("$ cd ..") {
            cur_tree.pop_back();
            continue;
        }

        if l.starts_with("$ cd") {
            let sp: Vec<&str> = l.split(" ").collect();
            cur_tree.push_back(&sp[2]);
            all_dirs.insert(path_from_deque(&cur_tree));
            continue;
        }

        if l.starts_with("$ ls") || l.starts_with("dir") {
            continue;
        }

        let s: Vec<&str> = l.split(" ").collect();
        let file_size: i32 = s[0].parse().expect("Should be a number");
        all_files_size.insert(path_for_file(&cur_tree, s[1]), file_size);
    }

    for k in all_dirs.iter() {
        let mut dir_size = 0;
        for f_kv in all_files_size.iter() {
            dir_size += if f_kv.0.starts_with(k) {f_kv.1} else {&0};
        }
        all_dirs_size.insert(k.to_string(), dir_size);
    }

    let mut score_1 = 0;
    for kv in all_dirs_size.iter() {
        if *kv.1 < 100000 {
            score_1 += *kv.1;
        }
    }
    println!("Score 1 = {}", score_1);

    let free_space = 70000000 - all_dirs_size["/"];
    let size_to_free = 30000000 - free_space;
    println!("Size to free: {}", size_to_free);
    let mut score_2 = 70000000;
    let mut d = String::from("");
    for kv in all_dirs_size.iter() {
        if *kv.1 < score_2 && *kv.1 > size_to_free {
            score_2 = *kv.1;
            d = kv.0.to_string();
        }
    }
    println!("Score 2 = {} @ {}", score_2, d);
}

pub fn solve_all() {
    solve("inputs/day_07_example_1.txt", "Example");
    solve("inputs/day_07.txt", "Main");
}