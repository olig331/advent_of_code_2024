use itertools::*;
use rayon::prelude::*;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
    time::Instant,
};

type Coords = (i32, i32);
type Grid = Vec<Vec<char>>;

const CONSTRAINTS: Coords = (71, 71);
const VECTORS: [Coords; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    let mut bench = Instant::now();
    let input = parse_input("input.txt");
    println!(
        "Part 1 - {} | Took {:?}",
        part1(&input, CONSTRAINTS),
        bench.elapsed()
    );
    bench = Instant::now();
    println!(
        "Part 2 - {:?} | Took {:?}",
        part2(&input, CONSTRAINTS),
        bench.elapsed()
    );
}

fn parse_input(path: &str) -> Vec<Coords> {
    let input = fs::read_to_string(path)
        .expect("Failed to read input...")
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    input
}

fn build_grid(input: Vec<Coords>, constraints: Coords) -> Grid {
    let mut grid = vec![vec!['.'; constraints.0 as usize]; constraints.1 as usize];

    for (x, y) in input.iter() {
        grid[*y as usize][*x as usize] = '#';
    }

    grid
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: Coords,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors(pos: Coords, grid: &Grid) -> Vec<Coords> {
    let (x, y) = pos;

    VECTORS
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|(new_x, new_y)| {
            *new_x >= 0
                && *new_x < 71
                && *new_y >= 0
                && *new_y < 71
                && grid[*new_y as usize][*new_x as usize] != '#'
        })
        .collect()
}

fn find_path(grid: &Grid, start: Coords, end: Coords) -> Option<u32> {
    let mut prio_q = BinaryHeap::new();
    let mut distances = HashMap::new();

    prio_q.push(Node {
        pos: start,
        cost: 0,
    });
    distances.insert(start, 0);

    while let Some(Node { pos, cost }) = prio_q.pop() {
        if pos == end {
            return Some(cost);
        }

        if let Some(&best) = distances.get(&pos) {
            if cost > best {
                continue;
            }
        }

        for neighbour in get_neighbors(pos, grid) {
            let next_cost = cost + 1;

            if !distances.contains_key(&neighbour)
                || next_cost < *distances.get(&neighbour).unwrap()
            {
                distances.insert(neighbour, next_cost);
                prio_q.push(Node {
                    pos: neighbour,
                    cost: next_cost,
                });
            }
        }
    }
    None
}

fn part1(input: &Vec<Coords>, constraints: Coords) -> u32 {
    let grid = build_grid(input[0..1024].to_vec(), constraints);
    let mut result = 0;

    match find_path(&grid, (0, 0), (70, 70)) {
        Some(cost) => result = cost,
        None => (),
    }
    result
}

fn part2(input: &Vec<Coords>, constraints: Coords) -> Coords {
    let result = (1023..3450)
        .into_par_iter()
        .map(|i| {
            let grid = build_grid(input[0..i].to_vec(), constraints);
            (i, find_path(&grid, (0, 0), (70, 70)))
        })
        .find_first(|(_, result)| result.is_none())
        .unwrap_or((0, None));

    input[result.0 - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse_input(TEST_INPUT_PATH), TEST_CONSTRAINTS), 22);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(
    //         part2(&parse_input(TEST_INPUT_PATH), TEST_CONSTRAINTS),
    //         (6, 1)
    //     );
    // }
}
