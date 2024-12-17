use std::{fs, i64, time::Instant};
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

struct Target {
    x: i64,
    y: i64,
}

fn parse(lines: &[&str], offset: i64) -> (Target, Target, Target) {
    let button_re = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
    let a_matches = button_re.captures(lines[0]).unwrap();
    let b_matches = button_re.captures(lines[1]).unwrap();
    let button_a = Target{ x: a_matches[1].parse().unwrap(), y: a_matches[2].parse().unwrap() };
    let button_b = Target{ x: b_matches[1].parse().unwrap(), y: b_matches[2].parse().unwrap() };

    let target_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let t_matches = target_re.captures(lines[2]).unwrap();
    let target = Target{ x: t_matches[1].parse::<i64>().unwrap() + offset, y: t_matches[2].parse::<i64>().unwrap() + offset };
    (button_a, button_b, target)
}

fn claw_game(button_a: Target, button_b: Target, target: Target) -> Option<i64> {
    // tx = ax * u + bx * v
    // ty = ay * u + by * v
    let tx = target.x;
    let ty = target.y;
    let ax = button_a.x;
    let ay = button_a.y;
    let bx = button_b.x;
    let by = button_b.y;
    (0..101).rev().cartesian_product(1..101).filter(|(a, b)| (tx == ax * a + bx * b) && (ty == ay * a + by * b)).map(|(a, b)| 3 * a + b).min()
}

fn big_claw_game(button_a: Target, button_b: Target, target: Target) -> Option<i64> {
    // tx = ax * u + bx * v
    // ty = ay * u + by * v
    // tx + ty = u * (ax + ay) + v * (bx + by)
    // (tx + ty) - v * (bx + by) = u * (ax + ay)
    // ((tx + ty) - v * (bx + by)) / (ax + ay) = u
    // u = ((by * v) - ty) / ay
    // v = ((ax * (((by * v) - ty) / ay)) - tx) / bx
    let tx = target.x;
    let ty = target.y;
    let ax = button_a.x;
    let ay = button_a.y;
    let bx = button_b.x;
    let by = button_b.y;
    let min_a = (tx / ax).min(ty / ay);
    let max_a = (tx / ax).max(ty / ay);
    let min_b = (tx / bx).min(ty / by);
    let max_b = (tx / bx).max(ty / by);
    (min_a..max_a).cartesian_product(min_b..max_b).filter(|(a, b)| (tx == ax * a + bx * b) && (ty == ay * a + by * b)).map(|(a, b)| 3 * a + b).min()
}

fn part1(lines: &Vec<&str>, expected: Option<i64>) -> i64 {
    // Implementation here
    let total: i64 = lines.chunks(4).map(|c| parse(c, 0)).map(|(a, b, t)| claw_game(a, b, t)).filter(|x| x.is_some()).map(|x| x.unwrap()).sum();
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn part2(lines: &Vec<&str>, expected: Option<i64>) -> i64  {
    // Implementation here
    let games: Vec<(Target, Target, Target)> = lines.chunks(4).map(|c| parse(c, 10000000000000)).collect();
    let total = games.into_par_iter().map(|(a, b, t)| big_claw_game(a, b, t)).filter(|x| x.is_some()).map(|x| x.unwrap()).sum();
    if let Some(exp) = expected {
        println!("Part 2: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn process_file(path: &str, exp1: Option<i64>, exp2: Option<i64>) -> (i64, i64) {
    let input = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = input.split('\n').into_iter().map(|l| {l.trim()}).collect();
    let now = Instant::now();
    let part1_result = part1(&lines, exp1);
    let p1 = now.elapsed();
    let part2_result = part2(&lines, exp2);
    let p2 = now.elapsed() - p1;
    println!("Part 1 time: {p1:.2?}  Part 2 time: {p2:.2?}");
    (part1_result, part2_result)
}

fn main() {
    let example_part1_expected = 480;
    let example_part2_expected = 1;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}