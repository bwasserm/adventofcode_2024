use std::{fs, iter::zip};


fn parse_lines(lines: &Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }
        let mut line = line.split_whitespace();
        left.push(line.next().unwrap_or("0").parse().unwrap_or(0));
        right.push(line.next().unwrap_or("0").parse().unwrap_or(0));
    }
    (left, right)
}

fn part1(lines: &Vec<&str>, expected: Option<i32>) -> i32 {
    // Implementation here
    let (mut left, mut right) = parse_lines(lines);
    left.sort();
    right.sort();
    let total = zip(left, right).map(|(l, r)| (r - l).abs()).fold(0, |acc, e| acc + e);

    if let Some(exp) = expected {
        println!("Part 1 Calculated: {total} expected: {exp}");
        // assert_eq!(exp, total);
    }
    total
}

fn part2(lines: &Vec<&str>, expected: Option<i32>) -> i32  {
    // Implementation here
    let (mut left, mut right) = parse_lines(lines);
    let mut total = 0;
    for l in left {
        total += l * right.iter().filter(|r| r == &&l).count() as i32;
    }
    if let Some(exp) = expected {
        println!("Part 2 Calculated: {total} expected: {exp}");
        // assert_eq!(exp, total);
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
    let example_part1_expected = 11;
    let example_part2_expected = 31;
    let (ex1, _) = process_file("example1", Some(example_part1_expected), None);
    let (_, ex2) = process_file("example1", None, Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}