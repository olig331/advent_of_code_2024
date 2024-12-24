use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1 - {}", part1(parse_input("input.txt")));
    println!("Part 2 - {}", part2(parse_input("test_input.txt")));
}

fn parse_input(path: &str) -> Vec<i64> {
    let nums = fs::read_to_string(path)
        .expect("Failed to read input...")
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    nums
}

fn mix(curr: &i64, next: i64) -> i64 {
    next ^ curr
}

fn prune(curr: &i64) -> i64 {
    curr % 16777216
}

fn part1(nums: Vec<i64>) -> i64 {
    let mut result = 0;
    for n in nums {
        let mut num = n;
        for _ in 0..2000 {
            let step1 = prune(&mix(&num, num * 64));
            let step2 = prune(&mix(&step1, step1 / 32));
            let step3 = prune(&mix(&step2, step2 * 2048));
            num = step3;
        }
        result += num;
    }
    result
}

fn last_digit(num: i64) -> i64 {
    num.to_string()
        .chars()
        .last()
        .unwrap()
        .to_string()
        .parse::<i64>()
        .unwrap()
}

fn part2(nums: Vec<i64>) -> i64 {
    let mut map = HashMap::new();

    for n in nums {
        let mut num = n;
        let mut seq = Vec::new();

        for _ in 0..2000 {
            let step1 = prune(&mix(&num, num * 64));
            let step2 = prune(&mix(&step1, step1 / 32));
            let step3 = prune(&mix(&step2, step2 * 2048));

            let next_last = last_digit(step3);
            let prev_last = last_digit(num);
            let diff = next_last - prev_last;

            seq.push(diff);
            num = step3;

            if seq.len() < 4 {
                continue;
            }

            let key = seq
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(",");

            match map.get_mut(&key) {
                Some(x) => *x += next_last,
                None => {
                    map.insert(key, next_last);
                }
            }

            seq.remove(0);
        }
    }

    let mut max = 0;
    let mut max_key = "".to_owned();

    for (k, v) in map {
        if v > max {
            max = v;
            max_key = k;
        }
    }

    println!("{} {}", max, max_key);

    max
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        todo!()
    }

    #[test]
    fn test_part2() {
        todo!()
    }
}
