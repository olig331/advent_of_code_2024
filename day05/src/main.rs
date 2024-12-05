use itertools::Itertools;
use std::{collections::HashMap, fs};

type TData = (HashMap<String, Vec<String>>, Vec<Vec<String>>);

fn main() {
    let (map, print_orders) = parse_input("input.txt");
    let (p1_result, failures) = part1(&map, print_orders);
    let p2_result = part2(&map, failures);

    println!("Part 1 - {}", p1_result);
    println!("Part 2 - {}", p2_result);
}

fn parse_input(path: &str) -> TData {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let input = fs::read_to_string(path)
        .expect("Could not get input...")
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    input[0].split("\n\n").for_each(|po| {
        let pages = po.split("|").collect::<Vec<&str>>();
        match map.get_mut(pages[0]) {
            Some(matched) => matched.push(pages[1].to_string()),
            None => {
                map.insert(pages[0].to_string(), vec![pages[1].to_string()]);
            }
        };
    });

    let print_orders = input[1]
        .split("\n")
        .map(|line| {
            line.split(",")
                .collect::<Vec<_>>()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    (map, print_orders)
}

fn check_order(map: &HashMap<String, Vec<String>>, print_order: &Vec<String>) -> bool {
    let mut passed = true;
    for (index, key) in print_order.iter().enumerate() {
        if index == print_order.len() - 1 {
            break;
        }
        match map.get(key) {
            Some(matched) => {
                for next in &print_order[index + 1..print_order.len()] {
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

fn part1(
    map: &HashMap<String, Vec<String>>,
    print_orders: Vec<Vec<String>>,
) -> (u32, Vec<Vec<String>>) {
    let mut result: u32 = 0;
    let mut failures = Vec::new();

    for order in print_orders {
        if check_order(&map, &order) {
            let idx: usize = order.len() / 2;
            result += order[idx].parse::<u32>().unwrap();
        } else {
            failures.push(order);
        }
    }
    (result, failures)
}

#[rustfmt::skip]
fn part2(map: &HashMap<String, Vec<String>>, failures: Vec<Vec<String>>) -> u32 {
    let mut result = 0;

    for failure in failures {
        let mut failure = failure.clone();
        let mut passed = false;

        while !passed {
            'swap_and_retry: for (update_index, map_key) in failure.iter().enumerate() {
                if update_index == failure.len() - 1 {
                    passed = true;
                    break;
                }
                match map.get(map_key) {
                    Some(pages) => {
                        for next_page in &failure[update_index + 1..failure.len()] {
                            if pages.contains(&next_page) {
                                continue;
                            }

                            passed = false;
                            let (idx, _) = failure.iter().find_position(|&k| k == next_page).unwrap();
                            failure.swap(update_index, idx);

                            break 'swap_and_retry;
                        }
                    }
                    None => {
                        failure.swap(update_index, update_index + 1);
                        passed = false;
                        break 'swap_and_retry;
                    }
                };
            }

            if passed {
                let idx = failure.len() / 2;
                result += failure[idx].parse::<u32>().unwrap();
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
        let (map, print_orders) = parse_input(TEST_INPUT_PATH);
        let (result, _) = part1(&map, print_orders);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part2() {
        let (map, print_orders) = parse_input(TEST_INPUT_PATH);
        let (_, failures) = part1(&map, print_orders);
        assert_eq!(part2(&map, failures), 123);
    }
}
