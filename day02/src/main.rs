use std::{fs};

fn parse_lines(lines: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = vec![];
    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }
        let levels = line.split_whitespace().map(|s| s.parse().unwrap_or(0)).collect();
        reports.push(levels);
    }
    reports
}

fn test_report(report: &Vec<i32>) -> bool {
    let up = report.windows(2).all(|ls| ls[0] < ls[1]);
    let down = report.windows(2).all(|ls| ls[0] > ls[1]);
    let small = report.windows(2).all(|ls| (ls[1] - ls[0]).abs() <= 3);
    small && (up ^ down)
}

fn part1(lines: &Vec<&str>, expected: Option<i32>) -> i32 {
    // Implementation here
    let reports = parse_lines(lines);
    let mut total = 0;
    for report in reports {
        if test_report(&report) {
            total += 1;
        }
    }
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn part2(lines: &Vec<&str>, expected: Option<i32>) -> i32  {
    // Implementation here
    let reports = parse_lines(lines);
    let mut total = 0;
    for report in reports {
        if test_report(&report) {
            total += 1;
        } else {
            for i in 0..report.len() {
                let mut r2: Vec<i32> = report[..i].to_vec();
                r2.extend_from_slice(&report[i+1..]);
                if test_report(&r2) {
                    total += 1;
                    break;
                }
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
    let lines: Vec<&str> = input.split('\n').into_iter().map(|l| {l.trim()}).collect();
    let part1_result = part1(&lines, exp1);
    let part2_result = part2(&lines, exp2);
    (part1_result, part2_result)
}

fn main() {
    let example_part1_expected = 2;
    let example_part2_expected = 4;
    let (ex1, _) = process_file("example1", Some(example_part1_expected), None);
    let (_, ex2) = process_file("example1", None, Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}