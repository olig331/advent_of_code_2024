use itertools::Itertools;
use std::{collections::HashMap, fs};

type TData = (HashMap<String, Vec<String>>, Vec<Vec<String>>);
type TCheckOrder = (HashMap<String, Vec<String>>, Vec<String>);

fn main() {
    let (map, instructions) = parse_input("input.txt");

    let (result, failures) = part1((map, instructions));
    let p2result = part2("input.txt", failures);
    println!("Part 1 - {}", result);
    println!("Part 2 - {}", p2result);
}

fn parse_input(path: &str) -> TData {
    let input = fs::read_to_string(path)
        .expect("Could not get input...")
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    input[0].split("\n").for_each(|po| {
        let pages = po.split("|").collect::<Vec<&str>>();
        match map.get_mut(pages[0]) {
            Some(matched) => matched.push(pages[1].to_string()),
            None => {
                map.insert(pages[0].to_string(), vec![pages[1].to_string()]);
            }
        };
    });

    let instructions = input[1]
        .split("\n")
        .map(|line| {
            line.split(",")
                .collect::<Vec<_>>()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    (map, instructions)
}

fn check_order(data: TCheckOrder) -> bool {
    let (map, order) = data;
    let mut passed = true;
    for (index, i) in order.iter().enumerate() {
        if index == order.len() - 1 {
            break;
        }
        match map.get(i) {
            Some(matched) => {
                for next in &order[index + 1..order.len()] {
                    if matched.contains(&next) {
                        continue;
                    }
                    passed = false;
                    break;
                }
            }
            None => {
                passed = false;
                break;
            }
        };
    }
    passed
}

fn part1(data: TData) -> (u32, Vec<Vec<String>>) {
    let (map, instructions) = data;
    let mut result: u32 = 0;
    let mut failures = Vec::new();

    for ins in instructions {
        if check_order((map.clone(), ins.clone())) {
            let indx = ((ins.len() / 2) as f32).floor() as usize;
            result += ins[indx].parse::<u32>().unwrap();
        } else {
            failures.push(ins);
        }
    }
    (result, failures)
}

fn part2(path: &str, failures: Vec<Vec<String>>) -> u32 {
    let (map, _) = parse_input(path);
    let mut result = 0;

    for ins in failures {
        let mut failure = ins.clone();
        let mut passed = false;
        while !passed {
            'retry: for (update_index, map_key) in failure.iter().enumerate() {
                if update_index == failure.len() - 1 {
                    passed = true;
                    break;
                }
                match map.get(map_key) {
                    Some(pages) => {
                        for (y, next_page) in
                            failure[update_index + 1..failure.len()].iter().enumerate()
                        {
                            if pages.contains(&next_page) {
                                continue;
                            }

                            passed = false;
                            let (idx, _) =
                                failure.iter().find_position(|&k| k == next_page).unwrap();
                            failure.swap(update_index, idx);

                            break 'retry;
                        }
                    }
                    None => {
                        failure.swap(update_index, update_index + 1);
                        passed = false;
                        break 'retry;
                    }
                };
            }

            if passed {
                let indx = ((failure.len() / 2) as f32).floor() as usize;
                result += failure[indx].parse::<u32>().unwrap();
                println!("Failure passed with new struct of - {:?}", failure);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT_PATH);
        let (result, _) = part1(input);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST_INPUT_PATH);
        let (_, failures) = part1(input);
        assert_eq!(part2(TEST_INPUT_PATH, failures), 123);
    }
}
