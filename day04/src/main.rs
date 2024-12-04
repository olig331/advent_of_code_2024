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

#[rustfmt::skip]
fn part2(input: &T2DVec) -> u32 {
    let mut result: u32 = 0;
    for (y, _) in input.iter().enumerate() {
        for (x, c) in input[y].iter().enumerate() {
            if c == &'A' {
                if y == 0 || y == input.len() - 1 || x == 0 || x == input[0].len() - 1 {
                    continue;
                }
            
                let w1: String = vec![input[(y - 1) as usize][(x - 1) as usize], 'A', input[(y + 1) as usize][(x + 1) as usize]].iter().collect();
                let w2: String = vec![input[(y + 1) as usize][(x - 1) as usize], 'A', input[(y - 1) as usize][(x + 1) as usize]].iter().collect();
            
                if (w1 == "MAS" || w1 == "SAM") && (w2 == "MAS" || w2 == "SAM") {
                    result += 1;
                }                
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
            if new_x >= 0 && new_x < input[0].len() as i32 && new_y >= 0 && new_y < input.len() as i32 {
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
