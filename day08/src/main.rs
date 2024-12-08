use itertools::Itertools;
use std::{collections::HashMap, fs, time::Instant};

type Coords = (i32, i32);

fn main() {
    let (antennas, bounds) = parse_input("input.txt");
    let p1bench = Instant::now();
    println!(
        "Part 1 - {} | Took {:?}",
        part1(&antennas, bounds),
        p1bench.elapsed()
    );
    let p2bench = Instant::now();
    println!(
        "Part 2 - {} | Took {:?}",
        part2(antennas, bounds),
        p2bench.elapsed()
    );
}

fn parse_input(path: &str) -> (HashMap<char, Vec<(i32, i32)>>, usize) {
    let mut antennas: HashMap<char, Vec<Coords>> = HashMap::new();
    let input = fs::read_to_string(path).expect("Failed to read input...");
    let bounds = input.lines().count();

    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, node)| {
            if node != '.' {
                match antennas.get_mut(&node) {
                    Some(matched) => matched.push((y.try_into().unwrap(), x.try_into().unwrap())),
                    None => {
                        antennas.insert(node, vec![(y.clone() as i32, x.clone() as i32)]);
                    }
                }
            }
        });
    });
    (antennas, bounds)
}

fn part1(antennas: &HashMap<char, Vec<(i32, i32)>>, bounds: usize) -> u32 {
    let mut result: Vec<Coords> = Vec::new();

    for (_, coords) in antennas.clone() {
        let combos: Vec<(Coords, Coords)> = coords.into_iter().tuple_combinations().collect();
        for (start, end) in &combos {
            let distance_y = start.0 - end.0;
            let distance_x = start.1 - end.1;
            let mut s_an = start.clone();
            let mut e_an = end.clone();

            if start.0 > end.0 {
                s_an.0 -= distance_y;
                e_an.0 += distance_y;
            } else {
                s_an.0 += distance_y;
                e_an.0 -= distance_y;
            }

            if start.1 > end.1 {
                s_an.1 += distance_x;
                e_an.1 -= distance_x;
            } else {
                s_an.1 += distance_x;
                e_an.1 -= distance_x;
            }

            if check_in_bounds(&s_an, bounds) && !result.contains(&s_an) {
                result.push(s_an);
            }

            if check_in_bounds(&e_an, bounds) && !result.contains(&e_an) {
                result.push(e_an);
            }
        }
    }
    result.len() as u32
}

fn check_in_bounds(pos: &Coords, bounds: usize) -> bool {
    pos.0 >= 0 && pos.0 <= (bounds as i32) - 1 && pos.1 >= 0 && pos.1 <= (bounds as i32) - 1
}

fn part2(antennas: HashMap<char, Vec<(i32, i32)>>, bounds: usize) -> u32 {
    let mut result: Vec<Coords> = Vec::new();

    for (_, coords) in antennas.clone() {
        let combos: Vec<(Coords, Coords)> = coords.into_iter().tuple_combinations().collect();
        for (start, end) in &combos {
            if !result.contains(start) {
                result.push(*start);
            };
            if !result.contains(end) {
                result.push(*end);
            }
            let distance_y = start.0 - end.0;
            let distance_x = start.1 - end.1;
            let mut s_an = start.clone();
            let mut e_an = end.clone();
            loop {
                if !check_in_bounds(&s_an, bounds) && !check_in_bounds(&e_an, bounds) {
                    break;
                }

                if start.0 > end.0 {
                    s_an.0 -= distance_y;
                    e_an.0 += distance_y;
                } else {
                    s_an.0 += distance_y;
                    e_an.0 -= distance_y;
                }

                if start.1 > end.1 {
                    s_an.1 += distance_x;
                    e_an.1 -= distance_x;
                } else {
                    s_an.1 += distance_x;
                    e_an.1 -= distance_x;
                }

                if check_in_bounds(&s_an, bounds) && !result.contains(&s_an) {
                    result.push(s_an);
                }

                if check_in_bounds(&e_an, bounds) && !result.contains(&e_an) {
                    result.push(e_an);
                }
            }
        }
    }
    result.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        let (antennas, bounds) = parse_input(TEST_INPUT_PATH);
        assert_eq!(part1(&antennas, bounds), 14)
    }

    #[test]
    fn test_part2() {
        let (antennas, bounds) = parse_input(TEST_INPUT_PATH);
        assert_eq!(part2(antennas, bounds), 34)
    }
}
