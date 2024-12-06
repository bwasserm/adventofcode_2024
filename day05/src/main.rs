use std::{cmp::Ordering, fs};

fn parse_file(lines: &Vec<&str>) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut split_lines = lines.split(|r| r == &"");
    // println!("{split_lines:?}");
    let rulelines = split_lines.next().unwrap();
    let updatelines = split_lines.next().unwrap();
    let mut rules: Vec<(i32, i32)> = vec![];
    for rl in rulelines {
        let mut rls = rl.split("|");
        let l: i32 = rls.next().unwrap().parse().unwrap();
        let r: i32 = rls.next().unwrap().parse().unwrap();
        rules.push((l, r));
    }
    // println!("rules: {rules:?}");
    let mut updates: Vec<Vec<i32>> = vec![];
    for ul in updatelines {
        let update: Vec<i32> = ul.split(",").map(|u| u.parse().unwrap()).collect();
        updates.push(update);
    }
    (rules, updates)
}

fn test_update(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> bool {
    let mut valid = true;
    for (i, r) in update.iter().enumerate() {
        for rule in rules.iter() {
            if rule.1 == *r {
                if update[i..].contains(&rule.0) {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            break;
        }
    }
    valid
}

fn part1(lines: &Vec<&str>, expected: Option<i32>) -> i32 {
    // Implementation here
    let (rules, updates) = parse_file(lines);
    let mut total = 0;
    for update in updates {
        let valid = test_update(&rules, &update);
        if valid {
            total += update[update.len() / 2];
        }
    }

    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn part2(lines: &Vec<&str>, expected: Option<i32>) -> i32  {
    // Implementation here
    let (rules, updates) = parse_file(lines);
    let mut total = 0;
    for update in updates {
        let valid = test_update(&rules, &update);
        if !valid {
            let mut update2 = update.clone();
            update2.sort_by(|a, b| {
                let mut ord: Ordering = Ordering::Equal;
                for rule in rules.iter() {
                    if &rule.0 == a && &rule.1 == b {
                        ord = Ordering::Less;
                        break;
                    } else if &rule.0 == b && &rule.1 == a {
                        ord = Ordering::Greater;
                        break;
                    }
                }
                ord
            });
            total += update2[update2.len() / 2];
        }
    }
    if let Some(exp) = expected {
        println!("Part 2: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn process_file(path: &str, exp1: Option<i32>, exp2: Option<i32>) -> (i32, i32) {
    let input = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = input.split('\n').into_iter().map(|l| {l.trim()}).collect();
    let part1_result = part1(&lines, exp1);
    let part2_result = part2(&lines, exp2);
    (part1_result, part2_result)
}

fn main() {
    let example_part1_expected = 143;
    let example_part2_expected = 123;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}