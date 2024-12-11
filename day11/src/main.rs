use std::{collections::BTreeMap, fs, time::Instant};

fn main() {
    let p1_bench = Instant::now();
    println!(
        "Part 1 - {} | Took {:?}",
        part1(parse_input("input.txt")),
        p1_bench.elapsed()
    );
    let p2_bench = Instant::now();
    println!(
        "Part 2 - {} | Took {:?}",
        part2(parse_input("input.txt")),
        p2_bench.elapsed()
    );
}

fn parse_input(path: &str) -> BTreeMap<u64, u64> {
    let mut map = BTreeMap::new();
    fs::read_to_string(path)
        .expect("Failed to read input...")
        .split_ascii_whitespace()
        .for_each(|s| {
            let num = s.parse::<u64>().unwrap();
            match map.get_mut(&num) {
                Some(count) => {
                    *count += 1 as u64;
                }
                None => {
                    map.insert(num, 1 as u64);
                }
            }
        });
    map
}

fn part1(map: BTreeMap<u64, u64>) -> u64 {
    let mut start_map = map.clone();

    for _ in 0..25 {
        for (item, count) in start_map.clone().iter() {
            if *count > 0 {
                if *item == 0 {
                    match start_map.get_mut(item) {
                        Some(x) => {
                            *x -= count;
                            match start_map.get_mut(&1) {
                                Some(y) => *y += *count,
                                None => {
                                    start_map.insert(1, *count);
                                }
                            }
                        }
                        None => (),
                    }

                    continue;
                }

                if item.to_string().len() % 2 == 0 {
                    let s = item.clone().to_string();
                    let mid = s.len() / 2;
                    let (first_half, second_half) = s.split_at(mid);
                    let parse_start = first_half.parse::<u64>();
                    let parse_end = second_half.parse::<u64>();

                    if parse_start.is_ok() {
                        match start_map.get_mut(&parse_start.clone().unwrap()) {
                            Some(ps) => {
                                *ps += *count;
                            }
                            None => {
                                start_map.insert(parse_start.clone().unwrap(), *count);
                            }
                        }
                    }

                    if parse_end.is_ok() {
                        match start_map.get_mut(&parse_end.clone().unwrap()) {
                            Some(ps) => {
                                *ps += *count;
                            }
                            None => {
                                start_map.insert(parse_end.clone().unwrap(), *count);
                            }
                        }
                    }

                    match start_map.get_mut(item) {
                        Some(x) => *x -= count,
                        None => todo!(),
                    }

                    continue;
                }

                match start_map.get_mut(item) {
                    Some(z) => {
                        *z -= *count;
                        let new_key = item * 2024 as u64;
                        match start_map.get_mut(&new_key) {
                            Some(a) => {
                                if *item == 2 {}

                                *a += *count;
                            }
                            None => {
                                start_map.insert(item * 2024, *count);
                            }
                        }
                    }
                    None => (),
                }
            }
        }
    }

    let mut sum = 0 as u64;
    for (_, val) in start_map.iter() {
        sum += val;
    }
    sum
}

fn part2(map: BTreeMap<u64, u64>) -> u64 {
    let mut start_map = map.clone();

    for _ in 0..75 {
        for (item, count) in start_map.clone().iter() {
            if *count > 0 {
                if *item == 0 {
                    match start_map.get_mut(item) {
                        Some(x) => {
                            *x -= count;
                            match start_map.get_mut(&1) {
                                Some(y) => *y += *count,
                                None => {
                                    start_map.insert(1, *count);
                                }
                            }
                        }
                        None => (),
                    }

                    continue;
                }

                if item.to_string().len() % 2 == 0 {
                    let s = item.clone().to_string();
                    let mid = s.len() / 2;
                    let (first_half, second_half) = s.split_at(mid);
                    let parse_start = first_half.parse::<u64>();
                    let parse_end = second_half.parse::<u64>();

                    if parse_start.is_ok() {
                        match start_map.get_mut(&parse_start.clone().unwrap()) {
                            Some(ps) => {
                                *ps += *count;
                            }
                            None => {
                                start_map.insert(parse_start.clone().unwrap(), *count);
                            }
                        }
                    }

                    if parse_end.is_ok() {
                        match start_map.get_mut(&parse_end.clone().unwrap()) {
                            Some(ps) => {
                                *ps += *count;
                            }
                            None => {
                                start_map.insert(parse_end.clone().unwrap(), *count);
                            }
                        }
                    }

                    match start_map.get_mut(item) {
                        Some(x) => *x -= count,
                        None => todo!(),
                    }

                    continue;
                }

                match start_map.get_mut(item) {
                    Some(z) => {
                        *z -= *count;
                        let new_key = item * 2024 as u64;
                        match start_map.get_mut(&new_key) {
                            Some(a) => {
                                if *item == 2 {}

                                *a += *count;
                            }
                            None => {
                                start_map.insert(item * 2024, *count);
                            }
                        }
                    }
                    None => (),
                }
            }
        }
    }

    let mut sum = 0 as u64;
    for (_, val) in start_map.iter() {
        sum += val;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(TEST_INPUT_PATH)), 55312);
    }
}
