#![feature(iter_intersperse)]
use std::{fs, ops::{BitXorAssign, Rem}, time::Instant, u64};
use rayon::prelude::*;
use regex::Regex;


fn combo(operand: &u64, a: &u64, b: &u64, c: &u64) -> u64 {
    match operand {
        0..4 => *operand,
        4 => *a,
        5 => *b,
        6 => *c,
        _ => panic!(),
    }
}

fn execute(opcode: &u64, operand: &u64, a: &mut u64, b: &mut u64, c: &mut u64, ip: &mut usize) -> Option<u64> {
    match opcode {
        0 => {  // adv
            *a = *a / 2u64.pow(combo(operand, a, b, c) as u32);
            *ip += 2;
            None
        },
        1 => {  // bxl
            b.bitxor_assign(operand);
            *ip += 2;
            None
        },
        2 => {  // bst
            *b = combo(operand, a, b, c).rem(8);
            *ip += 2;
            None
        },
        3 => {  // jnz
            if *a == 0 {
                *ip += 2;
            } else {
                *ip = *operand as usize;
            }
            None
        },
        4 => {  // bxc
            b.bitxor_assign(*c);
            *ip += 2;
            None
        },
        5 => {  // out
            *ip += 2;
            Some(combo(operand, a, b, c) % 8)
        },
        6 => {  // bdv
            *b = *a / 2u64.pow(combo(operand, a, b, c) as u32);
            *ip += 2;
            None
        },
        7 => {  // cdv
            *c = *a / 2u64.pow(combo(operand, a, b, c) as u32);
            *ip += 2;
            None
        },
        _ => panic!(),
    }
}

fn process(program: &Vec<u64>, a_starter: u64, b_starter: u64, c_starter: u64, quine_search: bool) -> Vec<u64> {
    let mut total: Vec<u64> = Vec::new();
    let mut ip = 0;
    let mut a: u64 = a_starter;
    let mut b: u64 = b_starter;
    let mut c: u64 = c_starter;
    // println!("IP: {ip} A: {a} B: {b} C: {c}");
    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];
        // println!("oc: {opcode} oa: {operand}");
        if let Some(output) = execute(&opcode, &operand, &mut a, &mut b, &mut c, &mut ip) {
            total.push(output);
            if quine_search {
                // println!("{a_starter} {:?} {:?}", total, program);
                if !total.iter().zip(program.iter()).all(|(t, p)| t == p) {
                    break;
                }
            }
        }
        // println!("IP: {ip} A: {a} B: {b} C: {c}");
    }
    total
}

fn part1(lines: &Vec<&str>, expected: Option<&str>, a_starter: Option<u64>) -> (String, bool) {
    let reg_re = Regex::new(r"Register .: (\d+)").unwrap();
    let mut a: u64 = reg_re.captures(lines[0]).unwrap().get(1).unwrap().as_str().parse().unwrap();
    if let Some(new_a) = a_starter {
        a = new_a;
    }
    let b: u64 = reg_re.captures(lines[1]).unwrap().get(1).unwrap().as_str().parse().unwrap();
    let c: u64 = reg_re.captures(lines[2]).unwrap().get(1).unwrap().as_str().parse().unwrap();
    let prog_re = Regex::new(r"Program: (.+)").unwrap();
    let program: Vec<u64> = prog_re.captures(lines[4]).unwrap().get(1).unwrap().as_str().split(",").map(|op| op.parse().unwrap()).collect();
    let total = process(&program, a, b, c, false);
    let matches = total == program;
    let totals = total.iter().map(|x| x.to_string()).intersperse(",".to_string()).collect();
    println!("Part 1: {totals}");
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {totals} Equal: {}", exp == totals);
    }
    (totals, matches)
}

fn part2(lines: &Vec<&str>, expected: Option<u64>) -> u64  {
    // Implementation here
    let reg_re = Regex::new(r"Register .: (\d+)").unwrap();
    let b: u64 = reg_re.captures(lines[1]).unwrap().get(1).unwrap().as_str().parse().unwrap();
    let c: u64 = reg_re.captures(lines[2]).unwrap().get(1).unwrap().as_str().parse().unwrap();
    let prog_re = Regex::new(r"Program: (.+)").unwrap();
    let program: Vec<u64> = prog_re.captures(lines[4]).unwrap().get(1).unwrap().as_str().split(",").map(|op| op.parse().unwrap()).collect();
    let total = (0u64..u64::MAX).into_par_iter().find_first(|a| {process(&program, *a, b, c, true) == program}).unwrap();
    if let Some(exp) = expected {
        println!("Part 2: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn process_file(path: &str, exp1: Option<&str>, exp2: Option<u64>) -> (String, u64) {
    let input = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = input.split('\n').into_iter().map(|l| {l.trim()}).collect();
    let now = Instant::now();
    let (part1_result, _) = part1(&lines, exp1, None);
    let p1 = now.elapsed();
    let part2_result = part2(&lines, exp2);
    let p2 = now.elapsed() - p1;
    println!("Part 1 time: {p1:.2?}  Part 2 time: {p2:.2?}");
    (part1_result, part2_result)
}

fn main() {
    let example_part1_expected = "4,6,3,5,6,3,5,2,1,0";
    let example_part2_expected = 117440;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}