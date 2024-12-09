use std::{collections::HashMap, fs, time::Instant};
use itertools::Itertools;
use std::ops::{Add, Sub};

fn load_map(lines: &Vec<&str>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (row, line) in lines.into_iter().enumerate() {
        for (col, sym) in line.chars().enumerate() {
            if sym != '.' {
                if antennas.contains_key(&sym) {
                    antennas.get_mut(&sym).unwrap().push((row, col));
                } else {
                    antennas.insert(sym, vec![(row, col)]);
                }
            }
        }
    }
    antennas
}

fn part1(lines: &Vec<&str>, expected: Option<i32>) -> i32 {
    // Implementation here
    let maxr = lines.len() as i32;
    let maxc = lines[0].len() as i32;
    let antennas = load_map(lines);
    let mut antinodes: Vec<(i32, i32)> = vec![];
    let ops: [fn(i32, i32)->i32; 2] = [i32::add, i32::sub];
    for locs in antennas.values() {
        for locvec in locs.iter().permutations(2) {
            let ar = locvec[0].0 as i32;
            let ac = locvec[0].1 as i32;
            let br = locvec[1].0 as i32;
            let bc = locvec[1].1 as i32;
            let rd = ar.abs_diff(br) as i32;
            let cd = ac.abs_diff(bc) as i32;
            for oi in 0..16 {
                let (oar, obr, oac, obc) = (ops[(oi >> 0) % 2], ops[(oi >> 1) % 2], ops[(oi >> 2) % 2], ops[(oi >> 3) % 2]);
                let ard = oar(ar, rd);
                let acd = oac(ac, cd);
                if ard == obr(br, 2 * rd) &&
                    acd == obc(bc, 2 * cd) &&
                    0 <= ard &&
                    ard < maxr &&
                    0 <= acd &&
                    acd < maxc {
                    antinodes.push((ard, acd));
                }
            }
        }
    }
    let total = antinodes.iter().unique().count() as i32;
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn part2(lines: &Vec<&str>, expected: Option<i32>) -> i32  {
    // Implementation here
    let maxr = lines.len() as i32;
    let maxc = lines[0].len() as i32;
    let antennas = load_map(lines);
    let mut antinodes: Vec<(i32, i32)> = vec![];
    let ops: [fn(i32, i32)->i32; 2] = [i32::add, i32::sub];
    for locs in antennas.values() {
        for locvec in locs.iter().permutations(2) {
            let ar = locvec[0].0 as i32;
            let ac = locvec[0].1 as i32;
            let br = locvec[1].0 as i32;
            let bc = locvec[1].1 as i32;
            let rd = ar.abs_diff(br) as i32;
            let cd = ac.abs_diff(bc) as i32;
            for oi in 0..16 {
                let (oar, obr, oac, obc) = (ops[(oi >> 0) % 2], ops[(oi >> 1) % 2], ops[(oi >> 2) % 2], ops[(oi >> 3) % 2]);
                for adist in 0..maxr {
                    let ard = oar(ar, adist * rd);
                    let acd = oac(ac, adist * cd);
                    for bdist in 0..maxr {
                        if ard == obr(br, bdist * rd) &&
                                acd == obc(bc, bdist * cd) &&
                                0 <= ard &&
                                ard < maxr &&
                                0 <= acd &&
                                acd < maxc {
                            antinodes.push((ard, acd));
                            // println!("{ard}, {acd}");
                        }
                    }
                }
            }
        }
    }
    let total = antinodes.iter().unique().count() as i32;
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
    let example_part1_expected = 14;
    let example_part2_expected = 34;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}