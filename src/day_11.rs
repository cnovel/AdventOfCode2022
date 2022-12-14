use std::fs;

struct Monkey {
    id: i32,
    div: i32,
    op: String,
    right_n: i32,
    monkey_true: i32,
    monkey_false: i32,
    inspect_count: i32
}

impl Monkey {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            div: 0,
            inspect_count: 0,
            monkey_false: -1,
            monkey_true: -1,
            op: "".to_string(),
            right_n: 0,
        }
    }

    fn new_worry(&self, worry: i32) -> i32 {
        if self.op == "*" {
            if self.right_n == -1 {
                return worry * worry;
            }
            return worry * self.right_n;
        }
        if self.right_n == -1 {
            return worry + worry;
        }
        return worry + self.right_n;
    }

    fn throw_item_to(&self, i: i32) -> i32 {
        if i % self.div == 0 {
            return self.monkey_true;
        }
        return self.monkey_false;
    }

    pub fn inspect_1(&mut self, items: &mut Vec<(i32, i32)>) {
        let mut new_items: Vec<(i32, i32)> = Vec::new();
        for item in items.iter_mut() {
            if item.0 != self.id {
                continue;
            }
            let mut worry = self.new_worry(item.1);
            worry /= 3;
            let next_monkey = self.throw_item_to(worry);
            new_items.push((next_monkey, worry));
            self.inspect_count += 1;
        }

        items.retain(|item| item.0 != self.id);
        items.append(&mut new_items);
    }

    pub fn inspect(&self, items: &mut Vec<Item>, monkeys: &Vec<Monkey>) -> i32 {
        let mut count = 0;
        for item in items.iter_mut() {
            if item.monkey_id != self.id {
                continue;
            }

            item.update_worry_with_monkeys(self.id, monkeys);
            if item.modulo_per_monkey[self.id as usize] == 0 {
                item.monkey_id = self.monkey_true;
            } else {
                item.monkey_id = self.monkey_false;
            }
            count += 1;
        }
        return count;
    }
}

struct Item {
    monkey_id: i32,
    modulo_per_monkey: Vec<i32>
}

impl Item {
    pub fn new(id: i32, worry: i32, monkeys: &Vec<Monkey>) -> Self {
        let mut mods = Vec::new();
        for m in monkeys {
            mods.push(worry % m.div);
        }

        return Self {
            monkey_id: id,
            modulo_per_monkey: mods
        }
    }

    pub fn update_worry_with_monkeys(&mut self, cur_monkey_id: i32, monkeys: &Vec<Monkey>) {
        let monkey = &monkeys[cur_monkey_id as usize];
        for j in 0..monkeys.len() {
            self.modulo_per_monkey[j] = monkey.new_worry(self.modulo_per_monkey[j]) % monkeys[j].div;
        }
    }
}

fn create_monkeys(contents: String, monkeys: &mut Vec<Monkey>, simple_items: &mut Vec<(i32, i32)>, items: &mut Vec<Item>) {
    for l in contents.split("\n").map(|l| l.trim()) {
        if l.is_empty() {
            continue;
        }
        if l.starts_with("Monkey") {
            monkeys.push(Monkey::new(monkeys.len() as i32));
            continue;
        }

        if l.starts_with("Starting items") {
            let new_items: Vec<i32> = l[16..].split(", ").map(|e| e.parse().expect("Should be a number")).collect();
            for i in new_items {
                simple_items.push((monkeys.last().unwrap().id, i));
            }
            continue;
        }

        if l.starts_with("Operation") {
            let s: Vec<&str> = l.split("=").collect();
            let formula: Vec<&str> = s.last().unwrap().trim().split(" ").collect();
            monkeys.last_mut().unwrap().op = formula[1].to_string();
            monkeys.last_mut().unwrap().right_n = if formula[2] == "old" { -1 } else { formula[2].parse().expect("Should be a number")};
            continue
        }

        if l.starts_with("Test") {
            let s: Vec<&str> = l.split(" ").collect();
            let div: i32 = s.last().unwrap().parse().expect("Should be a number");
            monkeys.last_mut().unwrap().div = div;
            continue;
        }

        if l.starts_with("If ") {
            let s: Vec<&str> = l.split(" ").collect();
            let monkey: i32 = s.last().unwrap().parse().expect("Should be a number");
            if l.starts_with("If true") {
                monkeys.last_mut().unwrap().monkey_true = monkey;

            } else {
                monkeys.last_mut().unwrap().monkey_false = monkey;
            }
            continue;
        }
    }

    for i in simple_items {
        items.push(Item::new(i.0, i.1, monkeys));
    }
}

fn get_max_product(monkeys: &Vec<Monkey>) -> i128 {
    let mut maxes = vec![0 as i128, 0 as i128];
    for m in monkeys {
        if m.inspect_count as i128 > maxes[1] {
            maxes[0] = maxes[1];
            maxes[1] = m.inspect_count as i128;
        } else if m.inspect_count as i128 > maxes[0] {
            maxes[0] = m.inspect_count as i128;
        }
    }
    return maxes[0]*maxes[1];
}

fn solve(p: &str, s: &str) {
    println!("Solving Day 11 - {}", s);
    let contents = fs::read_to_string(&p).expect("Should have been able to read the file");

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut simple_items: Vec<(i32, i32)> = Vec::new();
    let mut items: Vec<Item> = Vec::new();
    create_monkeys(contents, &mut monkeys, &mut simple_items, &mut items);

    // Part 1
    for _ in 0..20 {
        for m in &mut monkeys {
            m.inspect_1(&mut simple_items);
        }
    }
    println!("Score 1 = {}", get_max_product(&monkeys));
    for m in &mut monkeys {
        m.inspect_count = 0;
    }

    for _ in 0..10000 {
        let mut counts = Vec::new();
        for m in &monkeys {
            counts.push(m.inspect(&mut items, &monkeys));
        }
        for i in 0..monkeys.len() {
            monkeys[i].inspect_count += counts[i];
        }
    }
    println!("Score 2 = {}", get_max_product(&monkeys));
}

pub fn solve_all() {
    solve("inputs/day_11_example_1.txt", "Example");
    solve("inputs/day_11.txt", "Main");
}