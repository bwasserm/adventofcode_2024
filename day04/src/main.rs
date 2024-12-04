// https://adventofcode.com/2024/day/4

use std::fs;
use itertools::Itertools;

fn part1(lines: &Vec<&str>, expected: Option<i32>) -> i32 {
    // Implementation here
    let lines: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut total = 0;
    let mas = ['M', 'A', 'S'];
    for row_start in 0..num_rows {
        for col_start in 0..num_cols {
            if lines[row_start][col_start] == 'X' {
                for dir in vec![-1 as i32, -1, 0, 1, 1].iter().permutations(2).unique() {
                    for step in 0..3 {
                        let row: i32 = row_start as i32 + dir[0] * (step + 1) as i32;
                        let col: i32 = col_start as i32 + dir[1] * (step + 1) as i32;
                        if 0 <= row && row < num_rows as i32 && 0 <= col && col < num_cols as i32 {
                            if lines[row as usize][col as usize] == mas[step] {
                                if step == 2 {
                                    total += 1;
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    let total2 = (0..num_rows)
        .cartesian_product(0..num_cols)
        .filter(|(row, col)| lines[*row][*col] == 'X')
        .cartesian_product(vec![-1 as i32, -1, 0, 1, 1].iter().permutations(2).unique())
        .filter(|((row_start, col_start), dir)|
            (0..3).all(|step| {
                let row: i32 = *row_start as i32 + dir[0] * (step + 1) as i32;
                let col: i32 = *col_start as i32 + dir[1] * (step + 1) as i32;
                0 <= row 
                && row < num_rows as i32
                && 0 <= col
                && col < num_cols as i32
                && lines[row as usize][col as usize] == mas[step as usize]
            })
        )
        .count() as i32;
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
        println!("Part 1: Expected: {exp} Calculated: {total2} Equal: {}", exp == total2);
    }
    total2
}

fn part2(lines: &Vec<&str>, expected: Option<i32>) -> i32  {
    // Implementation here
    let lines: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let total = (1..(num_rows-1))
        .cartesian_product(1..(num_cols-1))
        .filter(|(row_start, col_start)| lines[*row_start][*col_start] == 'A')
        .filter(|(row_start, col_start)| 
            ((lines[row_start -  1][col_start - 1] == 'M' && lines[row_start +  1][col_start + 1] == 'S') ||
             (lines[row_start -  1][col_start - 1] == 'S' && lines[row_start +  1][col_start + 1] == 'M')) &&
            ((lines[row_start -  1][col_start + 1] == 'M' && lines[row_start +  1][col_start - 1] == 'S') ||
             (lines[row_start -  1][col_start + 1] == 'S' && lines[row_start +  1][col_start - 1] == 'M')))
        .count() as i32;
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
    let example_part1_expected = 18;
    let example_part2_expected = 9;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}