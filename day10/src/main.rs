use std::{fs, time::Instant};
use itertools::Itertools;

fn parse_map(lines: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut map: Vec<Vec<i32>> = vec![];
    for line in lines {
        map.push(line.chars().map(|c| c.to_string().parse().unwrap()).collect());
    }
    map
}

fn check_path(map: &Vec<Vec<i32>>, prev: i32, row: i32, col: i32) -> Vec<(i32, i32)> {
    if row < 0 || row >= map.len() as i32 || col < 0 || col >= map[0].len() as i32 {
        return vec![];
    }
    let here = map[row as usize][col as usize];
    if here != prev + 1 {
        // print!("{here}x");
        return vec![];
    } else {
        // print!("({prev}+1={here})");
    }
    if here == 9 {
        // println!("!({row}, {col})");
        return vec![(row, col)];
    }
    let mut peaks: Vec<(i32, i32)> = vec![];
    peaks.extend(check_path(map, here, row - 1, col));
    peaks.extend(check_path(map, here, row + 1, col));
    peaks.extend(check_path(map, here, row, col - 1));
    peaks.extend(check_path(map, here, row, col + 1));
    peaks
}


fn part1(lines: &Vec<&str>, expected: Option<i32>) -> i32 {
    // Implementation here
    let map = parse_map(lines);
    let mut total = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row as usize][col as usize] == 0 {
                total += check_path(&map, -1, row as i32, col as i32).iter().unique().count() as i32;
            }
        }
    }
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn part2(lines: &Vec<&str>, expected: Option<i32>) -> i32  {
    // Implementation here
    let map = parse_map(lines);
    let mut total = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row as usize][col as usize] == 0 {
                total += check_path(&map, -1, row as i32, col as i32).iter().count() as i32;
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
    let now = Instant::now();
    let part1_result = part1(&lines, exp1);
    let p1 = now.elapsed();
    let part2_result = part2(&lines, exp2);
    let p2 = now.elapsed() - p1;
    println!("Part 1 time: {p1:.2?}  Part 2 time: {p2:.2?}");
    (part1_result, part2_result)
}

fn main() {
    let example_part1_expected = 36;
    let example_part2_expected = 81;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}