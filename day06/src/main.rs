use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::{collections::HashMap, fs};

type Coords = (i32, i32);

fn main() {
    let (input, start) = parse_input("input.txt");
    let (p1_result, visited) = part1(&input, start);
    println!("Part 1 - {}", p1_result);
    println!("Part 2 - {}", part2(&input, start, visited));
}

fn parse_input(path: &str) -> (Vec<Vec<char>>, Coords) {
    let mut start: Coords = (0, 0);
    let input = fs::read_to_string(path)
        .expect("Failed to read input...")
        .lines()
        .enumerate()
        .map(|(y, row): (usize, &str)| {
            row.chars()
                .enumerate()
                .map(|(x, col)| {
                    if col == '^' {
                        start = (y as i32, x as i32);
                    }
                    col
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    (input, start)
}

#[derive(Debug, Copy, Clone)]
enum CurrentDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct DirectionState {
    current_direction: CurrentDirection,
    current_position: Coords,
    current_move: Coords,
}

impl DirectionState {
    fn new(start: Coords) -> Self {
        DirectionState {
            current_direction: CurrentDirection::Up,
            current_position: start,
            current_move: (-1, 0),
        }
    }
    fn set_position(&mut self, pos: Coords) {
        self.current_position = pos;
    }
    fn turn(&mut self) {
        match self.current_direction {
            CurrentDirection::Up => {
                self.current_move = (0, 1);
                self.current_direction = CurrentDirection::Right
            }
            CurrentDirection::Down => {
                self.current_move = (0, -1);
                self.current_direction = CurrentDirection::Left
            }
            CurrentDirection::Left => {
                self.current_move = (-1, 0);
                self.current_direction = CurrentDirection::Up
            }
            CurrentDirection::Right => {
                self.current_move = (1, 0);
                self.current_direction = CurrentDirection::Down
            }
        }
        ()
    }
}

#[rustfmt::skip]
fn part1(input: &Vec<Vec<char>>, start: Coords) -> (u32, HashMap<Coords, i32>) {
    let mut visited = HashMap::new();
    let mut state = DirectionState::new(start);
    visited.insert(start, 1);

    loop {
        let (new_y, new_x) = ( state.current_position.0 + state.current_move.0, state.current_position.1 + state.current_move.1);

        if new_y < 0 || new_y > (input.len() - 1) as i32 || new_x < 0 || new_x > (input[0].len() - 1) as i32 {
            break;
        }

        if input[new_y as usize][new_x as usize] == '#' {
            state.turn();
            continue;
        }

        match visited.get(&(new_y, new_x)) {
            None => {
                visited.insert((new_y, new_x), state.current_direction as i32);
                continue;
            }
            Some(_) => {}
        }
        state.set_position((new_y, new_x));
    }
    (visited.len() as u32, visited)
}

fn part2(input: &Vec<Vec<char>>, start: Coords, visited: HashMap<Coords, i32>) -> u32 {
    let result = AtomicU32::new(0);

    let visited_vec: Vec<_> = visited.into_iter().collect();
    visited_vec.into_par_iter().for_each(|node| {
        let mut state = DirectionState::new(start);
        let mut test_input = input.clone();
        if node.0 == start {
            return;
        }

        let (ny, nx) = node.0;

        test_input[ny as usize][nx as usize] = '#';

        let mut path_history: Vec<((i32, i32), i32)> = Vec::new();
        let mut turn_points: Vec<(Coords, i32)> = Vec::new();

        let mut test_res: Vec<Vec<char>> = test_input.clone();
        test_res[ny as usize][nx as usize] = '0';

        loop {
            let (new_y, new_x) = (
                state.current_position.0 + state.current_move.0,
                state.current_position.1 + state.current_move.1,
            );

            if new_y < 0
                || new_y > (input.len() - 1) as i32
                || new_x < 0
                || new_x > (input[0].len() - 1) as i32
            {
                break;
            }

            let mut freq_count = HashMap::new();
            for item in turn_points.iter() {
                *freq_count.entry(item.clone()).or_insert(0) += 1;
            }
            freq_count.retain(|_, c| *c > 1);
            if freq_count.len() >= 4 {
                if freq_count.len() >= 4 {
                    result.fetch_add(1, Ordering::SeqCst);
                }
                break;
            }

            if test_input[new_y as usize][new_x as usize] == '#' {
                state.turn();
                turn_points.push(((new_y, new_x), state.current_direction as i32));
                continue;
            }

            state.set_position((new_y, new_x));
            path_history.push(((new_y, new_x), state.current_direction as i32));
        }
    });

    result.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        let (input, start) = parse_input(TEST_INPUT_PATH);
        let (p1_result, _) = part1(&input, start);
        assert_eq!(p1_result, 41);
    }

    #[test]
    fn test_part2() {
        let (input, start) = parse_input(TEST_INPUT_PATH);
        let (_, visited) = part1(&input, start);
        let p2_result = part2(&input, start, visited);
        assert_eq!(p2_result, 41);
    }
}
