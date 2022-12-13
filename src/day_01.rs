use std::fs;

fn solve(p: &str, s: &str) {
    println!("Solving Day 01 - {}", s);

    let contents = fs::read_to_string(&p)
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
    println!("Max calories for the three best elves: {} [{}]",
        max_calories_per_elf[0..3].iter().sum::<i32>(),
        max_calories_per_elf[0..3].iter().map( |&id| id.to_string()).collect::<Vec<String>>().join(" + "));
}

pub fn solve_all() {
    solve("inputs/day_01_example_1.txt", "Example");
    solve("inputs/day_01.txt", "Main");
}