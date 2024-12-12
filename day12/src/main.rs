use std::{fs, time::Instant};

fn explore_region(lines: &Vec<Vec<char>>, plot: char, row: usize, col: usize, visited: &mut Vec<Vec<bool>>) -> (i32, i32) {
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut area = 1;
    let mut perimeter = 0;
    visited[row][col] = true;
    if row == 0 {
        perimeter += 1;
    } else {
        if lines[row - 1][col] != plot {
            perimeter += 1;
        } else if visited[row - 1][col] == false {
            let (a, p) = explore_region(lines, plot, row - 1, col, visited);
            area += a;
            perimeter += p;
        }
    }
    if row == num_rows - 1 {
        perimeter += 1;
    } else {
        if lines[row + 1][col] != plot {
            perimeter += 1;
        } else if visited[row + 1][col] == false {
            let (a, p) = explore_region(lines, plot, row + 1, col, visited);
            area += a;
            perimeter += p;
        }
    }
    if col == 0 {
        perimeter += 1;
    } else {
        if lines[row][col - 1] != plot {
            perimeter += 1;
        } else if visited[row][col - 1] == false {
            let (a, p) = explore_region(lines, plot, row, col - 1, visited);
            area += a;
            perimeter += p;
        }
    }
    if col == num_cols - 1 {
        perimeter += 1;
    } else {
        if lines[row][col + 1] != plot {
            perimeter += 1;
        } else if visited[row][col + 1] == false {
            let (a, p) = explore_region(lines, plot, row, col + 1, visited);
            area += a;
            perimeter += p;
        }
    }
    (area, perimeter)
}

fn part1(lines: &Vec<&str>, expected: Option<i32>) -> i32 {
    // Implementation here
    let mut areas: Vec<i32> = vec![];
    let mut perimeters: Vec<i32> = vec![];
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];
    let map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    for (row, line) in lines.iter().enumerate() {
        for (col, plot) in line.chars().enumerate() {
            if visited[row][col] == false {
                let (area, perimeter) = explore_region(&map, plot, row, col, &mut visited);
                // println!("Region {plot} starting at ({row}, {col}) A: {area} P: {perimeter}");
                areas.push(area);
                perimeters.push(perimeter);
            }
        }
    }
    let total = areas.iter().zip(perimeters.iter()).map(|(a, p)| a * p).sum::<i32>();
    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn walk_perimeter(lines: &Vec<Vec<char>>, plot: char, row: usize, col: usize, visited: &mut Vec<Vec<bool>>) -> i32 {
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut perimeter = 0;
    visited[row][col] = true;
    if row == 0 {
        perimeter += 1;
    } else {
        if lines[row - 1][col] != plot {
            perimeter += 1;
        } else if visited[row - 1][col] == false {
            let p = walk_perimeter(lines, plot, row - 1, col, visited);
            perimeter += p;
        }
    }
    if row == num_rows - 1 {
        perimeter += 1;
    } else {
        if lines[row + 1][col] != plot {
            perimeter += 1;
        } else if visited[row + 1][col] == false {
            let (a, p) = explore_region(lines, plot, row + 1, col, visited);
            perimeter += p;
        }
    }
    if col == 0 {
        perimeter += 1;
    } else {
        if lines[row][col - 1] != plot {
            perimeter += 1;
        } else if visited[row][col - 1] == false {
            let (a, p) = explore_region(lines, plot, row, col - 1, visited);
            perimeter += p;
        }
    }
    if col == num_cols - 1 {
        perimeter += 1;
    } else {
        if lines[row][col + 1] != plot {
            perimeter += 1;
        } else if visited[row][col + 1] == false {
            let (a, p) = explore_region(lines, plot, row, col + 1, visited);
            perimeter += p;
        }
    }
    perimeter
}


fn part2(lines: &Vec<&str>, expected: Option<i32>) -> i32  {
    // Implementation here
    let mut areas: Vec<i32> = vec![];
    let mut perimeters: Vec<i32> = vec![];
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut area_visited: Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];
    let mut peri_visited: Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];
    let map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    for (row, line) in lines.iter().enumerate() {
        for (col, plot) in line.chars().enumerate() {
            if area_visited[row][col] == false {
                let (area, _) = explore_region(&map, plot, row, col, &mut area_visited);
                let perimeter = walk_perimeter(&map, plot, row, col, &mut peri_visited);
                // println!("Region {plot} starting at ({row}, {col}) A: {area} P: {perimeter}");
                areas.push(area);
                perimeters.push(perimeter);
            }
        }
    }
    let total = areas.iter().zip(perimeters.iter()).map(|(a, p)| a * p).sum::<i32>();
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
    let example_part1_expected = 1930;
    let example_part2_expected = 1;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}