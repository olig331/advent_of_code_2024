use std::fs;

fn main() {
    println!("Result - {}", part1(parse_input("input.txt")));
}

fn parse_input(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path)
        .expect("Failed to read input...")
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<String>>();
    input
}

fn part1(vecs: Vec<String>) -> usize {
    let mut keys: Vec<[i8; 5]> = Vec::new();
    let mut locks: Vec<[i8; 5]> = Vec::new();

    for v in vecs {
        let grid = v.lines().collect::<Vec<_>>();
        if grid.first() == Some(&"#####") && grid.last() == Some(&".....") {
            let mut result: [i8; 5] = [-1; 5];
            for (i, line) in v.lines().enumerate() {
                if i == 0 {
                    continue;
                }

                for (x, c) in line.chars().enumerate() {
                    if c == '.' && result[x] == -1 {
                        result[x] = i as i8 - 1;
                    }
                }
            }
            locks.push(result);
        }

        if grid.last() == Some(&"#####") && grid.first() == Some(&".....") {
            let mut result: [i8; 5] = [-1; 5];
            for (i, line) in v.lines().rev().enumerate() {
                if i == 0 {
                    continue;
                }

                for (x, c) in line.chars().enumerate() {
                    if c == '.' && result[x] == -1 {
                        result[x] = i as i8 - 1;
                    }
                }
            }
            keys.push(result);
        }
    }

    let mut count = 0;
    for key in &keys {
        for lock in &locks {
            if lock.iter().enumerate().all(|(i, v)| key[i] + v <= 5) {
                count += 1
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(TEST_INPUT_PATH)), 3)
    }
}
