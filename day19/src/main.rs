use std::{collections::HashMap, fs};

fn main() {
    let (towels, patterns) = parse_input("input.txt");
    println!("Part 1 - {}", part1(&towels, &patterns));
    println!("Part 2 - {}", part2(&towels, &patterns))
}

fn parse_input(path: &str) -> (Vec<String>, Vec<String>) {
    let binding = fs::read_to_string(path).expect("Failed to read input...");
    let (towels, patterns) = binding.split_once("\n\n").unwrap();
    let towels = towels
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let patterns = patterns.lines().map(|s| s.to_string()).collect::<Vec<_>>();

    (towels, patterns)
}

fn num_of_ways(target: &str, towels: &Vec<String>, mut seen: &mut HashMap<String, usize>) -> usize {
    if let Some(prev) = seen.get(target) {
        return *prev;
    }

    let next = towels
        .iter()
        .filter(|p| target.starts_with(*p))
        .map(|p| {
            let (_, rest) = target.split_at(p.len());
            if rest.len() == 0 {
                1
            } else {
                num_of_ways(rest, towels, &mut seen)
            }
        })
        .sum();

    seen.insert(target.to_string(), next);
    next
}

fn can_make(target: &str, towels: &Vec<String>) -> bool {
    if target.is_empty() {
        return true;
    }
    for t in towels {
        if target.starts_with(t) {
            if can_make(&target[t.len()..], towels) {
                return true;
            }
        }
    }
    false
}

fn part1(towels: &Vec<String>, patterns: &Vec<String>) -> usize {
    patterns
        .iter()
        .filter(|t| can_make(t, &towels))
        .collect::<Vec<_>>()
        .len()
}

fn part2(towels: &Vec<String>, patterns: &Vec<String>) -> usize {
    let mut cache = HashMap::new();
    patterns
        .iter()
        .map(|p| num_of_ways(&p, towels, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        let (towels, patterns) = parse_input(TEST_INPUT_PATH);
        assert_eq!(part1(&towels, &patterns), 6)
    }

    #[test]
    fn test_part2() {
        let (towels, patterns) = parse_input(TEST_INPUT_PATH);
        assert_eq!(part2(&towels, &patterns), 16)
    }
}
