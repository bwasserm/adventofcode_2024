use std::{fs, time::Instant};
use std::ops::{Add, Mul};

fn part1(lines: &Vec<&str>, expected: Option<u64>) -> u64 {
    // Implementation here
    let mut total = 0;
    for line in lines {
        let mut terms = line.split_ascii_whitespace().into_iter();
        let pottot = terms.next().unwrap();
        let pottot: u64 = pottot[0..(pottot.len()-1)].parse().unwrap();
        let cals: Vec<u64> = terms.map(|t| t.parse().unwrap()).collect();
        let ops: [fn(u64, u64)->u64; 2] = [u64::add, u64::mul];
        // for i in 0..(2u32.pow(cals.len() as u32 - 1)) {
        //     let i = i as usize;
        //     let newtot = cals.iter().skip(1).zip(0..cals.len()).zip(0..(2u32.pow(cals.len() as u32 - 1))).fold(cals[0], |acc, ((cal, op_idx), i)| ops[(i >> op_idx) % 2](acc, *cal));
        //     // println!("{pottot} {i} {newtot}");
        //     if newtot == pottot {
        //         total += pottot;
        //         break;
        //     }
        // }
        if (0..(2u32.pow(cals.len() as u32 - 1))).any(|i| pottot == cals.iter().skip(1).zip(0..cals.len()).fold(cals[0], |acc, (cal, op_idx)| ops[(i >> op_idx) as usize % 2](acc, *cal))) {
            total += pottot;
        }
    }
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn part2(lines: &Vec<&str>, expected: Option<u64>) -> u64  {
    // Implementation here
    let mut total = 0;
    for line in lines {
        let mut terms = line.split_ascii_whitespace().into_iter();
        let pottot = terms.next().unwrap();
        let pottot: u64 = pottot[0..(pottot.len()-1)].parse().unwrap();
        let cals: Vec<u64> = terms.map(|t| t.parse().unwrap()).collect();
        let ops: [fn(u64, u64)->u64; 3] = [u64::add, u64::mul, |a, b| (a.to_string() + b.to_string().as_ref()).parse().unwrap()];
        // for i in 0..(3u32.pow(cals.len() as u32 - 1)) {
        //     let i = i as usize;
        //     let newtot = cals.iter().skip(1).zip(0..cals.len()).fold(cals[0], |acc, (cal, op_idx)| ops[(i / 3usize.pow(op_idx as u32)) % 3](acc, *cal));
        //     // println!("{pottot} {i} {newtot}");
        //     if newtot == pottot {
        //         total += pottot;
        //         break;
        //     }
        // }
        // About the same runtime, slightly slower than above loop
        if (0..(3u32.pow(cals.len() as u32 - 1))).any(|i| pottot == cals.iter().skip(1).zip(0..cals.len()).fold(cals[0], |acc, (cal, op_idx)| ops[(i / 3u32.pow(op_idx as u32)) as usize % 3](acc, *cal))) {
            total += pottot;
        }
    }
    if let Some(exp) = expected {
        println!("Part 2: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn process_file(path: &str, exp1: Option<u64>, exp2: Option<u64>) -> (u64, u64) {
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
    let example_part1_expected = 3749;
    let example_part2_expected = 11387;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}