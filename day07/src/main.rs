use core::panic;
use itertools::*;
use rayon::prelude::*;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

fn main() {
    let input = parse_input("input.txt");
    let p1_bench = Instant::now();
    println!("Part 1 - {} | Took {:?}", part1(&input), p1_bench.elapsed());

    let p2_bench = Instant::now();
    println!("Part 2 - {} | Took {:?}", part2(&input), p2_bench.elapsed());
}

fn parse_input(path: &str) -> Vec<(u64, Vec<u64>)> {
    let input = fs::read_to_string(path)
        .expect("Could not read input...")
        .lines()
        .map(|l| {
            let (target, nums) = l.split_once(": ").unwrap();
            (
                target.parse::<u64>().unwrap(),
                nums.trim()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .collect::<Vec<(u64, Vec<u64>)>>();
    input
}

fn part1(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut result: u64 = 0;

    for (target, nums) in input {
        let num_of_ops = nums.len() - 1;

        let mut all_combos = (0..num_of_ops)
            .map(|_| ['*', '+'])
            .multi_cartesian_product();

        let valid = all_combos.any(|combo| {
            let mut test = combo.iter();

            let combo_value = nums
                .iter()
                .copied()
                .reduce(|a, b| match test.next().unwrap() {
                    '*' => a * b,
                    '+' => a + b,
                    _ => panic!(""),
                })
                .unwrap();

            *target == combo_value
        });

        if valid == true {
            result += target;
        }
    }

    result
}

fn part2(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    let result = AtomicU64::new(0);

    input.into_par_iter().for_each(|(target, nums)| {
        let num_of_ops = nums.len() - 1;

        let mut all_combos = (0..num_of_ops)
            .map(|_| ["*", "+", "||"])
            .multi_cartesian_product();

        let valid = all_combos.any(|combo| {
            let mut test = combo.iter();

            let combo_value = nums
                .iter()
                .copied()
                .reduce(|a, b| match test.next().unwrap() {
                    &"*" => a * b,
                    &"+" => a + b,
                    &"||" => format!("{}{}", a, b).parse::<u64>().unwrap(),
                    _ => panic!(""),
                })
                .unwrap();

            *target == combo_value
        });

        if valid {
            result.fetch_add(*target, Ordering::Relaxed);
        };
    });

    result.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_PATH)), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_PATH)), 11387);
    }
}
