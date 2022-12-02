use std::fs;

struct Problem {
    input_path: String
}

fn solve(p: &Problem, s: &str) {
    println!("Solving Day 01 - {}", s);

    let contents = fs::read_to_string(&p.input_path)
        .expect("Should have been able to read the file");

    let mut max_calories_per_elf: Vec<i32> = Vec::new(); 
    let mut cur_calories = 0;

    for x in contents.split("\n") {
        if x.trim().len() == 0 {
            max_calories_per_elf.push(cur_calories);
            cur_calories = 0;
            continue;
        }
        let c: i32 = x.trim().parse().expect("Should be a number");
        cur_calories += c;
    }
    max_calories_per_elf.push(cur_calories); // Last one
    max_calories_per_elf.sort();
    max_calories_per_elf.reverse();
    println!("Max calories for the best elf: {}", max_calories_per_elf[0]);
    println!("Max calories for the three best elves: {} [{} + {} + {}]", 
        max_calories_per_elf[0] + max_calories_per_elf[1] + max_calories_per_elf[2],
        max_calories_per_elf[0], max_calories_per_elf[1], max_calories_per_elf[2]);
    println!("----------");
}

pub fn solve_all() {
    let p_ex = Problem {
        input_path: "inputs/day_01_example_1.txt".to_string()
    };
    solve(&p_ex, "Example");
    let p = Problem {
        input_path: "inputs/day_01.txt".to_string()
    };
    solve(&p, "Main");
}