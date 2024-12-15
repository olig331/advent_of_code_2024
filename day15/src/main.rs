use itertools::*;
use std::{fs, time::Instant};

type Coords = (i32, i32);
const LEFT: Coords = (0, -1);
const RIGHT: Coords = (0, 1);
const UP: Coords = (-1, 0);
const DOWN: Coords = (1, 0);

fn main() {
    let bench = Instant::now();
    println!(
        "Part 1 - {} | Took {:?}",
        part1("input.txt"),
        bench.elapsed()
    );
    // println!("Part 2 - {}", part2());
}

fn parse_input(path: &str) -> (Vec<Vec<char>>, Vec<char>, Coords) {
    let mut start = (0, 0);
    let binding = fs::read_to_string(path).expect("Failed to read input...");
    let (grid, moves) = binding.split("\n\n").collect_tuple().unwrap();
    (
        grid.lines()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '@' {
                            start = (y as i32, x as i32);
                        }
                        c
                    })
                    .collect()
            })
            .collect(),
        moves.chars().filter(|c| *c != '\n').collect(),
        start,
    )
}

fn handle_cases(mut grid: &mut Vec<Vec<char>>, pos: &mut Coords, dir: Coords) {
    let height: i32 = grid.len() as i32;
    let width: i32 = grid[0].len() as i32;
    let (ny, nx) = (pos.0 + dir.0, pos.1 + dir.1);

    if !out_of_bounds(*pos, width, height) {
        match &grid[ny as usize][nx as usize] {
            '#' => (),
            '.' => {
                grid[pos.0 as usize][pos.1 as usize] = '.';
                *pos = (ny, nx);
                grid[ny as usize][nx as usize] = '@';
            }
            'O' => {
                let result = move_robot(&mut grid, (ny, nx), dir);
                if result {
                    grid[pos.0 as usize][pos.1 as usize] = '.';
                    *pos = (ny, nx);
                }
            }
            _ => (),
        }
    }
}

fn part1(path: &str) -> usize {
    let (mut grid, moves, start) = parse_input(path);
    let mut pos = start;
    for m in moves {
        match m {
            '>' => handle_cases(&mut grid, &mut pos, RIGHT),
            '<' => handle_cases(&mut grid, &mut pos, LEFT),
            '^' => handle_cases(&mut grid, &mut pos, UP),
            'v' => handle_cases(&mut grid, &mut pos, DOWN),
            _ => (),
        }
    }

    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &c)| if c == 'O' { 100 * y + x } else { 0 })
        })
        .sum()
}

fn move_robot(grid: &mut Vec<Vec<char>>, pos: Coords, dir: Coords) -> bool {
    let mut to_move = Vec::new();
    let mut can_move = true;
    let (mut my, mut mx) = (pos.0, pos.1);
    while can_move {
        if !out_of_bounds((my, mx), grid[0].len() as i32, grid.len() as i32) {
            if grid[my as usize][mx as usize] == 'O' {
                to_move.push((my, mx));
                my += dir.0;
                mx += dir.1;
                continue;
            }
            if grid[my as usize][mx as usize] == '.' {
                to_move.push((my, mx));
                can_move = true;
                break;
            }
            if grid[my as usize][mx as usize] == '#' {
                can_move = false;
                break;
            }
        }
        can_move = false;
        break;
    }

    if can_move {
        let (y, x) = to_move.last().unwrap();
        let (fy, fx) = to_move[0];
        grid[fy as usize][fx as usize] = '@';
        grid[*y as usize][*x as usize] = 'O';
        return true;
    }
    return false;
}

fn out_of_bounds(pos: Coords, width: i32, height: i32) -> bool {
    pos.0 <= 0 || pos.0 >= height - 1 || pos.1 <= 0 || pos.1 >= width - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        todo!()
    }

    #[test]
    fn test_part2() {
        todo!()
    }
}
