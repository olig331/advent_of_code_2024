use std::fs;

type T2DVec = Vec<Vec<char>>;
type Coords = (i32, i32);

fn main() {
    println!("Part 1 - {}", part1(&parse_input("input.txt")));
    println!("Part 2 - {}", part2(&parse_input("input.txt")));
}

fn parse_input(path: &str) -> T2DVec {
    let content = fs::read_to_string(&path)
        .expect("Failed to read input...")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    content
}

fn part1(input: &T2DVec) -> u32 {
    let mut result: u32 = 0;
    for (y, _) in input.iter().enumerate() {
        for (x, c) in input[y as usize].iter().enumerate() {
            if c == &'S' || c == &'X' {
                result += make_word(input, (x as i32, y as i32));
            }
        }
    }
    result / 2
}

fn part2(input: &T2DVec) -> u32 {
    let mut result: u32 = 0;
    for (y, _) in input.iter().enumerate() {
        for (x, c) in input[y as usize].iter().enumerate() {
            if c == &'A' {
                result += make_mas(input, (x as i32, y as i32));
            }
        }
    }
    result
}

#[rustfmt::skip]
fn make_word(input: &T2DVec, (x, y): Coords) -> u32 {    
    let directions: Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, 1), (0, -1), (1, -1), (1, 0),(1, 1)];
    let (mut new_x, mut new_y) = (x, y);
    let mut result: u32 = 0;

    for dir in directions {
        let mut word: Vec<char> = Vec::new();
        word.push(input[y as usize][x as usize]);
        for _ in 1..=3 {
            (new_x += dir.0, new_y += dir.1);
            if new_x >= 0 && new_x < input[y as usize].len().try_into().unwrap() && new_y >= 0 && new_y < input.len().try_into().unwrap() {
                word.push(input[new_y as usize][new_x as usize]);
                continue;
            }
            break;
        }
        (new_x = x, new_y = y);    
        if word.iter().collect::<String>() == "XMAS" || word.iter().collect::<String>() == "SAMX" {
            result += 1;
        }
    }
    result
}

fn make_mas(input: &T2DVec, (x, y): Coords) -> u32 {
    let diag1 = vec![(-1, -1), (1, 1)];
    let diag2 = vec![(1, -1), (-1, 1)];

    let (mut new_x, mut new_y) = (x, y);
    let mut result: u32 = 0;

    let mut word1: Vec<char> = vec!['A'];
    let mut word2: Vec<char> = vec!['A'];

    for (vx, vy) in diag1 {
        (new_x += vx, new_y += vy);
        if new_x >= 0
            && new_x < input[y as usize].len().try_into().unwrap()
            && new_y >= 0
            && new_y < input.len().try_into().unwrap()
        {
            word1.push(input[new_y as usize][new_x as usize]);
        }
        (new_x = x, new_y = y);
    }

    for (vx, vy) in diag2 {
        (new_x += vx, new_y += vy);
        if new_x >= 0
            && new_x < input[y as usize].len().try_into().unwrap()
            && new_y >= 0
            && new_y < input.len().try_into().unwrap()
        {
            word2.push(input[new_y as usize][new_x as usize]);
        }
        (new_x = x, new_y = y);
    }

    let w1 = word1.iter().collect::<String>();
    let w2 = word2.iter().collect::<String>();
    if (w1 == "AMS" || w1 == "ASM") && (w2 == "AMS" || w2 == "ASM") {
        result += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_PATH)), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_PATH)), 9);
    }
}
