use std::{fs, time::Instant};

fn read_map(lines: &Vec<&str>) -> (Vec<Vec<bool>>, (usize, usize)) {
    let mut map: Vec<Vec<bool>> = vec![];
    let mut guard_pos: (usize, usize) = (0, 0);
    for (r, line) in lines.iter().enumerate() {
        let mut row: Vec<bool> = vec![];
        for (c, sym) in line.as_bytes().iter().enumerate() {
            match sym {
                b'.' => row.push(false),
                b'#' => row.push(true),
                b'^' => {
                    row.push(false);
                    guard_pos = (r, c);
                },
                _ => { println!("Unknown map symbol {sym} at {r},{c}");}
            };
        }
        map.push(row);
    }
    (map, guard_pos)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn check_map(map: &Vec<Vec<bool>>, mut r: usize, mut c: usize) -> (Vec<Vec<bool>>, bool) {
    let rmax = map.len();
    let cmax = map[0].len();
    let mut dir: Dir = Dir::Up;
    let mut exited = false;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cmax]; rmax];
    let mut visitdir: Vec<Vec<Option<Vec<Dir>>>> = vec![vec![None; cmax]; rmax];
    let mut looped = false;
    while !exited {
        visited[r][c] = true;
        if visitdir[r][c].is_some() {
            if visitdir[r][c].as_ref().unwrap().contains(&dir) {
                looped = true;
                break;
            } else {
                visitdir[r][c].as_mut().unwrap().push(dir.clone());
            }
        } else {
            let mut newdirs = vec![];
            newdirs.push(dir.clone());
            visitdir[r][c] = Some(newdirs);
        }
        dir = match dir {
            Dir::Up => {
                if r == 0 {
                    exited = true;
                    Dir::Up
                } else if map[r - 1][c] {
                    Dir::Right
                } else {
                    r -= 1;
                    Dir::Up
                }
            },
            Dir::Right => {
                if c + 1 == cmax {
                    exited = true;
                    Dir::Right
                } else if map[r][c + 1] {
                    Dir::Down
                } else {
                    c += 1;
                    Dir::Right
                }
            },
            Dir::Down => {
                if r + 1 == rmax {
                    exited = true;
                    Dir::Down
                } else if map[r + 1][c] {
                    Dir::Left
                } else {
                    r += 1;
                    Dir::Down
                }
            },
            Dir::Left => {
                if c == 0 {
                    exited = true;
                    Dir::Left
                } else if map[r][c - 1] {
                    Dir::Up
                } else {
                    c -= 1;
                    Dir::Left
                }
            }
        }
    }
    (visited, looped)
}

fn part1(lines: &Vec<&str>, expected: Option<i32>) -> i32 {
    // Implementation here
    let (map, (r, c)) = read_map(lines);
    let (visited, _) = check_map(&map, r, c);
    let total: usize = visited.iter().map(|r| r.iter().filter(|c| **c).count()).sum();
    let total = total as i32;
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn part2(lines: &Vec<&str>, expected: Option<i32>) -> i32  {
    // Implementation here
    let mut total: i32 = 0;
    let (map, (gr, gc)) = read_map(lines);
    let rmax = map.len();
    let cmax = map[0].len();
    let (visited, _) = check_map(&map, gr, gc);
    for r in 0..rmax {
        for c in 0..cmax {
            if visited[r][c] && (r, c) != (gr, gc) {
                let mut newmap = map.clone();
                newmap[r][c] = true;
                let (_, looped) = check_map(&newmap, gr, gc);
                if looped {
                    total += 1;
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
    let now = Instant::now();
    let part1_result = part1(&lines, exp1);
    let p1 = now.elapsed();
    let part2_result = part2(&lines, exp2);
    let p2 = now.elapsed() - p1;
    println!("Part 1 time: {p1:.2?}  Part 2 time: {p2:.2?}");
    (part1_result, part2_result)
}

fn main() {
    let example_part1_expected = 41;
    let example_part2_expected = 6;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}