use std::{fs, time::Instant};


#[derive(Debug)]
struct File {
    id: u64,
    size: u32,
    start: usize,
}

#[derive(Debug)]
struct Free {
    size: u32,
    start: usize,
}

#[derive(Debug)]
enum Fs {
    File(File),
    Free(Free),
}

fn part1(lines: &Vec<&str>, expected: Option<u64>) -> u64 {
    // Implementation here
    let mut fs: Vec<Fs> = vec![];
    let mut id = 0;
    let mut isfile = true;
    let mut ridx: u64 = 0;
    for (idx, val) in lines[0].chars().map(|c| c.to_string().parse().unwrap()).enumerate() {
        if isfile {
            fs.push(Fs::File(File{id, size: val, start: idx}));
            ridx += val as u64;
            id += 1;
        } else {
            fs.push(Fs::Free(Free{size: val, start: idx}));
            ridx += val as u64;

        }
        isfile = !isfile;
    }
    println!("Len {}", fs.len());
    let mut fwditer = fs.iter();
    let mut reviter = fs.iter().rev();
    let mut total: u64 = 0;
    let end = fs.iter().map(|f| {
        match f {
            Fs::File(f) => f.size as u64,
            Fs::Free(f) => f.size as u64,
        }
    }).sum();
    println!("ridx: {ridx} end: {end}");
    let mut idx: u64 = 0;
    let mut fwd = fwditer.next();
    let mut rev = reviter.next();
    let mut revi = 0;
    let mut done = false;
    while idx < end && !done {
        match fwd {
            Some(Fs::File(f)) => {
                for i in 0..f.size {
                    if ridx <= idx {
                        // println!("Rev idx: {ridx} before idx: {idx}");
                        done = true;
                        break;
                    }
                    total += idx * f.id;
                    // println!("{idx} * f{}", f.id);
                    idx += 1;
                }
            },
            Some(Fs::Free(f)) => {
                let mut fi = 0;
                while fi < f.size{
                    match rev {
                        Some(Fs::File(r)) => {
                            if ridx < idx {
                                // println!("Rev idx: {ridx} before idx: {idx}");
                                done = true;
                                break;
                            }
                            if revi < r.size {
                                total += idx * r.id;
                                // println!("{idx} * r{}", r.id);
                                idx += 1;
                                revi += 1;
                                fi += 1;
                                ridx -= 1;
                            } else {
                                rev = reviter.next();
                                revi = 0;
                            }
                        },
                        Some(Fs::Free(r)) => {
                            ridx -= r.size as u64;
                            rev = reviter.next();
                            revi = 0;
                        },
                        None => break
                    };
                }
            },
            None => break
        };
        fwd = fwditer.next();
    }    

    if let Some(exp) = expected {
        println!("Part 1: Expected: {exp} Calculated: {total} Equal: {}", exp == total);
    }
    total
}

fn part2(lines: &Vec<&str>, expected: Option<u64>) -> u64  {
    // Implementation here
    let total = lines.len() as u64;
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
    let example_part1_expected = 1928;
    let example_part2_expected = 2858;
    let (ex1, ex2) = process_file("example", Some(example_part1_expected), Some(example_part2_expected));
    println!("Example 1: {ex1} 2: {ex2}");
    let (p1, p2) = process_file("input", None, None);
    println!("Input: part 1: {p1} part 2: {p2}");
}