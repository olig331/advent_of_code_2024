use std::{collections::HashMap, fs};

type Coords = (usize, usize);

fn main() {
    let (input, start_positions) = parse_input("input.txt");

    println!("Part 1 - {}", part1(&input, &start_positions));
    println!("Part 2 - {}", part2(&input, &start_positions));
}

fn parse_input(path: &str) -> (Vec<Vec<u32>>, Vec<Coords>) {
    let mut start_positions = Vec::new();
    let input = fs::read_to_string(path)
        .expect("Failed to read input...")
        .lines()
        .enumerate()
        .map(|(y, r)| {
            r.chars()
                .enumerate()
                .map(|(x, c)| {
                    let num = c.to_digit(10).unwrap() as u32;
                    if num == 0 {
                        start_positions.push((y, x));
                    }
                    num
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<_>>>();

    (input, start_positions)
}

const VECTORS: [(i16, i16); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn recursion(
    start: Coords,
    input: &Vec<Vec<u32>>,
    curr_pos: Coords,
    map: &mut HashMap<Coords, Vec<Coords>>,
) {
    let (y, x) = curr_pos;
    let curr_val = &input[y][x];

    if *curr_val == 9 {
        match map.get_mut(&start) {
            Some(matched) => matched.push((y, x)),
            None => {
                map.insert(start, vec![(y, x)]);
            }
        };
        return ();
    }

    let next_val = curr_val + 1;

    for (vy, vx) in VECTORS {
        let ny = y as i16 + vy;
        let nx = x as i16 + vx;

        if nx >= 0 && nx < input[0].len() as i16 && ny >= 0 && ny < input.len() as i16 {
            if input[ny as usize][nx as usize] == next_val {
                recursion(
                    start,
                    &input,
                    (ny.try_into().unwrap(), nx.try_into().unwrap()),
                    map,
                );
            }
        }
    }

    ()
}

fn part1(input: &Vec<Vec<u32>>, start_positions: &Vec<Coords>) -> u64 {
    let mut result = HashMap::new();
    for (y, x) in start_positions {
        recursion((*y, *x), &input, (*y, *x), &mut result);
    }

    let mut sum = 0 as u64;
    for (_, val) in result.iter() {
        let mut copy = val.clone();
        copy.sort();
        copy.dedup();
        sum += copy.len() as u64
    }

    sum
}

fn part2(input: &Vec<Vec<u32>>, start_positions: &Vec<Coords>) -> u64 {
    let mut result = HashMap::new();
    for (y, x) in start_positions {
        recursion((*y, *x), &input, (*y, *x), &mut result);
    }

    let mut sum = 0;
    for (_, val) in result.iter() {
        sum += val.len() as u64;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        let (input, start_positions) = parse_input(TEST_INPUT_PATH);
        assert_eq!(part1(&input, &start_positions), 36)
    }

    #[test]
    fn test_part2() {
        let (input, start_positions) = parse_input(TEST_INPUT_PATH);
        assert_eq!(part2(&input, &start_positions), 81)
    }
}
