use std::{collections::BTreeMap, fs, time::Instant};

#[rustfmt::skip]
fn main() {
    let p1_bench = Instant::now();
    println!( "Part 1 - {} | Took {:?}", solve(parse_input("input.txt"), 25), p1_bench.elapsed());
    let p2_bench = Instant::now();
    println!( "Part 2 - {} | Took {:?}", solve(parse_input("input.txt"), 75), p2_bench.elapsed());
}

#[rustfmt::skip]
fn parse_input(path: &str) -> BTreeMap<u64, u64> {
    let mut map = BTreeMap::new();
    fs::read_to_string(path)
        .expect("Failed to read input...")
        .split_ascii_whitespace()
        .for_each(|s| {
            let num = s.parse::<u64>().unwrap();
            match map.get_mut(&num) {
                Some(count) =>  *count += 1 as u64,
                None => { map.insert(num, 1 as u64); }
            }
        });
    map
}

fn condition_one(start_map: &mut BTreeMap<u64, u64>, item: u64, count: u64) {
    if let Some(x) = start_map.get_mut(&item) {
        *x -= count;
        *start_map.entry(1).or_insert(0) += count;
    }
}

fn condition_two(start_map: &mut BTreeMap<u64, u64>, item: u64, count: u64) {
    let s = item.to_string();
    let (first, second) = s.split_at(s.len() / 2);
    for half in [first, second] {
        if let Ok(num) = half.parse::<u64>() {
            *start_map.entry(num).or_insert(0) += count;
        }
    }
    *start_map.get_mut(&item).expect("...") -= count;
}

fn condition_three(start_map: &mut BTreeMap<u64, u64>, item: u64, count: u64) {
    if let Some(current) = start_map.get_mut(&item) {
        *current -= count;
        *start_map.entry(item * 2024).or_insert(0) += count;
    }
}

#[rustfmt::skip]
fn solve(map: BTreeMap<u64, u64>, range: usize) -> u64 {
    let mut start_map = map.clone();
    for _ in 0..range {
        for (item, count) in start_map.clone().iter() {
            if *count == 0 { 
                continue 
            };
            
            match item {
                item if *item == 0 => condition_one(&mut start_map, *item, *count),                
                item if item.to_string().len() % 2 == 0 => condition_two(&mut start_map, *item, *count),
                _ => condition_three(&mut start_map, *item, *count),
            }
        }
    }
    start_map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        assert_eq!(solve(parse_input(TEST_INPUT_PATH), 25), 55_312);
    }
}
