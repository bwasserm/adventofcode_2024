// https://adventofcode.com/2024/day/3
// https://xkcd.com/208/

use std::fs;
use regex::Regex;


fn part1(lines: &str, expected: Option<i32>) -> i32 {
    // Implementation here
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    for (_, [first, second]) in re.captures_iter(lines).map(|c| c.extract()) {
        let first = first.parse().unwrap_or(0);
        let second = second.parse().unwrap_or(0);
        total += first * second;
    }
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn part2(lines: &str, expected: Option<i32>) -> i32  {
    // Implementation here
    let domulre = Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))").unwrap();
    let mulre = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    let mut enabled = true;
    for (_, [action]) in domulre.captures_iter(lines).map(|c| c.extract()) {
        if action == "do()" {
            enabled = true;
            continue;
        }
        else if action == "don't()" {
            enabled = false;
            continue;
        }
        else if enabled {
            for (_, [first, second]) in mulre.captures_iter(action).map(|c| c.extract()) {
                let first = first.parse().unwrap_or(0);
                let second = second.parse().unwrap_or(0);
                total += first * second;
            }
        }
    }
    if let Some(exp) = expected {
        println!("Part 2: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn process_file(path: &str, exp1: Option<i32>, exp2: Option<i32>) -> (i32, i32) {
    let input = fs::read_to_string(path).unwrap();
    //let lines: Vec<&str> = input.split('\n').into_iter().map(|l| {l.trim()}).collect();
    let part1_result = part1(&input, exp1);
    let part2_result = part2(&input, exp2);
    (part1_result, part2_result)
}

fn main() {
    let example_part1_expected = 161;
    let example_part2_expected = 48;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}