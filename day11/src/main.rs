use std::{collections::HashMap, fs, time::Instant};
use rayon::prelude::*;

fn count_stones(lines: &Vec<&str>, blinks: usize) -> i32 {
    let mut stones: Vec<u64> = lines[0].split_whitespace().map(|s| s.parse().unwrap()).collect();
    for b in 0..blinks {
        let mut next_stones = vec![];
        next_stones.par_extend(stones.into_par_iter().map(|stone| {
            if stone == 0 {
                vec![1]
            } else {
                let numdigs = stone.ilog10() + 1;
                if numdigs % 2 == 0 {
                    let half_digits: u64 = 10u64.pow(numdigs / 2);
                    vec![stone / half_digits, stone % half_digits]
                } else {
                    vec![stone * 2024]
                }
            }
        }));
        // println!("{next_stones:?}");
        stones = next_stones.concat();
        // println!("{b}: {}", stones.len());
    }
    stones.len() as i32
}

fn count_stones2(stones: &[u64], blinks: usize, known: &mut HashMap<u64, u64>) -> u64 {
    if stones.iter().all(|s| known.contains_key(s)) {
        return stones.iter().map(|s| known.get(s).unwrap()).sum();
    }
    if blinks == 0 {
        let num_stones = stones.len() as u64;
        let _ = stones.iter().map(|s| known.insert(*s, num_stones));
        return num_stones;
    }
    stones.iter().map(|stone| {
        if stone == &0 {
            count_stones2(&[1], blinks - 1, known)
        } else {
            let numdigs = stone.ilog10() + 1;
            if numdigs % 2 == 0 {
                let half_digits: u64 = 10u64.pow(numdigs / 2);
                count_stones2(&[stone / &half_digits, stone % half_digits], blinks - 1, known)
            } else {
                count_stones2(&[stone * 2024], blinks - 1, known)
            }
        }
    }).sum()
}

fn part1(lines: &Vec<&str>, expected: Option<i32>) -> i32 {
    // Implementation here
    let total = count_stones(lines, 25);
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn part2(lines: &Vec<&str>, expected: Option<i32>) -> i32  {
    // Implementation here
    let stones: Vec<u64> = lines[0].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut known: HashMap<u64, u64> = HashMap::new();
    let total = count_stones2(stones.iter().as_slice(), 75, &mut known) as i32;
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
    // let example_part1_expected = 55312;
    // let example_part2_expected = 55312;
    // let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    // println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}