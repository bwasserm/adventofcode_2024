use std::{env, fs};


fn part1(lines: &Vec<&str>, expected: Option<i64>) -> i64 {
    let mut total = lines.len() as i64;
    // Implementation here
    if let Some(exp) = expected {
        assert_eq!(exp, total);
    }
    total
}

fn part2(lines: &Vec<&str>, expected: Option<i64>) -> i64  {
    let mut total = lines.len() as i64;
    // Implementation here
    if let Some(exp) = expected {
        assert_eq!(exp, total);
    }
    total
}

fn process_file(path: &str, exp1: Option<i64>, exp2: Option<i64>) -> (i64, i64) {
    let input = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = input.split('\n').into_iter().map(|l| {l.trim()}).collect();
    let part1_result = part1(&lines, exp1);
    let part2_result = part2(&lines, exp2);
    (part1_result, part2_result)
}

fn main() {
    let example_part1_expected = 4;
    let example_part2_expected = 7;
    let (ex1, _) = process_file("example1", Some(example_part1_expected), None);
    let (_, ex2) = process_file("example2", None, Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}