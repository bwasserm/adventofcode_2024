use std::{fs::{self, File}, io::{self, Write}, time::Instant};
use regex::Regex;


fn part1(lines: &Vec<&str>, width: i32, height: i32, expected: Option<i32>) -> i32 {
    let steps = 100;
    let robot_re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut bl: i32 = 0;
    let mut br: i32 = 0;
    let mut tl: i32 = 0;
    let mut tr: i32 = 0;
    for line in lines {
        let m = robot_re.captures(line).unwrap();
        let start_col: i32 = m[1].parse().unwrap();
        let start_row: i32 = m[2].parse().unwrap();
        let cvel: i32 = m[3].parse().unwrap();
        let rvel: i32 = m[4].parse().unwrap();
        let row = (start_row + steps * rvel).rem_euclid(height);
        let col = (start_col + steps * cvel).rem_euclid(width);
        if row < height / 2 && col < width / 2{
            tl += 1;
        } else if row > height / 2 && col < width / 2 {
            bl += 1;
        } else if row < height / 2 && col > width / 2 {
            tr += 1;
        } else if row > height / 2 && col > width / 2 {
            br += 1;
        }
    }
    println!("{tl} {tr}\n{bl} {br}");
    let total = bl * br * tl * tr;
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

struct Robot {
    row: i32,
    col: i32,
    rvel: i32,
    cvel: i32,
}

impl Robot {
    fn run(self: &Self, steps: i32, width: i32, height: i32) -> (usize, usize) {
        let row = (self.row + steps * self.rvel).rem_euclid(height);
        let col = (self.col + steps * self.cvel).rem_euclid(width);
        (row as usize, col as usize)
    }
}

fn part2(lines: &Vec<&str>, width: i32, height: i32, expected: Option<i32>) -> i32  {
    // Implementation here
    let robot_re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<Robot> = vec![];
    for line in lines {
        let m = robot_re.captures(line).unwrap();
        let col: i32 = m[1].parse().unwrap();
        let row: i32 = m[2].parse().unwrap();
        let cvel: i32 = m[3].parse().unwrap();
        let rvel: i32 = m[4].parse().unwrap();
        robots.push(Robot{row, col, rvel, cvel});
    }
    let mut file = File::create("output").unwrap();
    for i in 0..(101 * 103) {
        if i % 101 != 4 && i % 103 != 76 {
            continue;
        }
        let mut map: Vec<Vec<bool>> = vec![vec![false; width as usize]; height as usize];
        for robot in robots.iter() {
            let (r, c) = robot.run(i, width, height);
            map[r][c] = true;
        }
        write!(file, "Step: {i}\n").unwrap();
        for r in 0..height {
            for c in 0..width {
                if map[r as usize][c as usize] {
                    write!(file, "x").unwrap();
                } else {
                    write!(file, " ").unwrap();
                }
            }
            write!(file, "\n").unwrap();
        }
    }

    let total = lines.len() as i32;
    if let Some(exp) = expected {
        println!("Part 2: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn process_file(path: &str, width: i32, height: i32, exp1: Option<i32>, exp2: Option<i32>) -> (i32, i32) {
    let input = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = input.split('\n').into_iter().map(|l| {l.trim()}).collect();
    let now = Instant::now();
    let part1_result = part1(&lines, width, height, exp1);
    let p1 = now.elapsed();
    let part2_result = part2(&lines, width, height, exp2);
    let p2 = now.elapsed() - p1;
    println!("Part 1 time: {p1:.2?}  Part 2 time: {p2:.2?}");
    (part1_result, part2_result)
}

fn main() {
    let example_part1_expected = 12;
    let example_part2_expected = 1;
    // let (ex1, ex2) = process_file("example", 11, 7, Some(example_part1_expected), Some(example_part2_expected));
    // println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", 101, 103, None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}