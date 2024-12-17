#![feature(slice_split_once)]
use std::{fs, time::Instant};

#[derive(PartialEq, Copy, Clone)]
enum MapObj {
    Robot,
    Wall,
    Box,
    Empty,
}

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn part1(lines: &Vec<&str>, expected: Option<i32>) -> i32 {
    // Implementation here
    let (map_lines, moves_lines) = lines.split_once(|l| l == &"").unwrap();
    let num_rows = map_lines.len() - 1;
    let num_cols = moves_lines[0].len() - 1;
    let mut row = 0;
    let mut col = 0;
    let mut map: Vec<Vec<MapObj>> = map_lines.iter().enumerate().map(|(r, line)| line.chars().enumerate().map(|(c, s)| match s {
            '@' => {row = r; col = c; MapObj::Robot},
            '#' => MapObj::Wall,
            'O' => MapObj::Box,
            '.' => MapObj::Empty,
            _ => MapObj::Wall,
        }).collect()).collect();
    let moves = moves_lines.concat();
    let moves = moves.chars().map(|c| match c {
        '^' => Move::Up,
        'v' => Move::Down,
        '<' => Move::Left,
        '>' => Move::Right,
        _ => Move::Right,
    });
    for m in moves {
        match m {
            Move::Up => {
                match map[row - 1][col] {
                    MapObj::Empty => row = row - 1,
                    MapObj::Box => {
                        for r in (row - 2)..0 {
                            if map[r][col] == MapObj::Empty {
                                for r2 in r..row {
                                    map[r2][col] = map[r2 + 1][col];
                                }
                                break;
                            }
                            if map[r][col] == MapObj::Wall {
                                break;
                            }
                        }
                    }
                    MapObj::Wall => {},
                    MapObj::Robot => {},
                }
            },
            Move::Down => {
                match map[row + 1][col] {
                    MapObj::Empty => row = row + 1,
                    MapObj::Box => {
                        for r in (row + 2)..num_rows {
                            if map[r][col] == MapObj::Empty {
                                for r2 in r..row {
                                    map[r2][col] = map[r2 - 1][col];
                                }
                                break;
                            }
                            if map[r][col] == MapObj::Wall {
                                break;
                            }
                        }
                    }
                    MapObj::Wall => {},
                    MapObj::Robot => {},
                }
            },
            Move::Left => {
                match map[row][col - 1] {
                    MapObj::Empty => col = col - 1,
                    MapObj::Box => {
                        for c in (col - 2)..0 {
                            if map[row][c] == MapObj::Empty {
                                for c2 in c..col {
                                    map[row][c2] = map[row][c2 - 1];
                                }
                                break;
                            }
                            if map[row][c] == MapObj::Wall {
                                break;
                            }
                        }
                    }
                    MapObj::Wall => {},
                    MapObj::Robot => {},
                }
            },
            Move::Right => {
                match map[row][col + 1] {
                    MapObj::Empty => col = col + 1,
                    MapObj::Box => {
                        for c in (col + 2)..num_cols {
                            if map[row][c] == MapObj::Empty {
                                for c2 in c..col {
                                    map[row][c2] = map[row][c2 + 1];
                                }
                                break;
                            }
                            if map[row][c] == MapObj::Wall {
                                break;
                            }
                        }
                    }
                    MapObj::Wall => {},
                    MapObj::Robot => {},
                }
            },
        }
    }
    let mut total = 0;
    for (r, row) in map.iter().enumerate() {
        for (c, mo) in row.iter().enumerate() {
            match mo {
                MapObj::Robot => print!("@"),
                MapObj::Box => {
                    total += 100 * r + c;
                    print!("O")
                },
                MapObj::Wall => print!("#"),
                MapObj::Empty => print!("."),
            }
        }
        println!("");
    }
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total as i32);
    }
    total as i32
}

fn part2(lines: &Vec<&str>, expected: Option<i32>) -> i32  {
    // Implementation here
    let total = lines.len() as i32;
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
    let example_part1_expected = 10092;
    let example_part2_expected = 1;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}