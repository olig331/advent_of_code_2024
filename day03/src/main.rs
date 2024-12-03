use regex::Regex;
use std::fs;

fn main() {
    println!("Part 1 - {}", part1(parse_input("input.txt")));
    println!("Part 2 - {}", part2(parse_input("input.txt")))
}

fn parse_input(path: &str) -> String {
    let input = fs::read_to_string(path).expect("Failed to read input...");
    input
}

fn part1(input: String) -> u32 {
    let reg = Regex::new(r"(mul\(\d\d?\d?,\d\d?\d?\))").unwrap();
    let mut result = 0;
    for c in reg.find_iter(&input) {
        let parts = c.as_str().split(",").collect::<Vec<_>>();
        let sums = parts
            .iter()
            .map(|p| {
                let re = Regex::new(r"\d+").unwrap();
                let result = re.find_at(&p, 0).unwrap().as_str().parse::<u32>().unwrap();
                result
            })
            .collect::<Vec<u32>>();

        result += sums[0] * sums[1];
    }
    result
}

enum ShouldParse {
    Parse,
    NoParse,
}

fn part2(input: String) -> u32 {
    let reg = Regex::new(r"(mul\(\d\d?\d?,\d\d?\d?\))|(?<dont>don't\(\))|(?<do>do\(\))").unwrap();
    let mut should_parse = ShouldParse::Parse;
    let mut result = 0;

    for c in reg.captures_iter(&input) {
        match c.name("do") {
            Some(_) => {
                should_parse = ShouldParse::Parse;
                continue;
            }
            None => (),
        }
        match c.name("dont") {
            Some(_) => {
                should_parse = ShouldParse::NoParse;
                continue;
            }
            None => (),
        }

        match should_parse {
            ShouldParse::Parse => result += part1(c.get(0).unwrap().as_str().to_string()),
            ShouldParse::NoParse => (),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";
    const PART2_TEST_INPUT_PATH: &str = "part2_test_input.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(TEST_INPUT_PATH)), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(PART2_TEST_INPUT_PATH)), 48);
    }
}
