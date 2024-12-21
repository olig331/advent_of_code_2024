use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
};

type Coords = (i32, i32);
type Grid = Vec<Vec<char>>;
type TInput = (Grid, Coords, Coords);
const VECTORS: [Coords; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    println!("Part 1 - {}", part1(parse_input("input.txt")));
    println!("Part 2 - {}", part2());
}

fn parse_input(path: &str) -> TInput {
    let (mut start, mut end) = ((0, 0), (0, 0));
    let input = fs::read_to_string(path)
        .expect("Failed to read input...")
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        'S' => start = (y as i32, x as i32),
                        'E' => end = (y as i32, x as i32),
                        _ => (),
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    (input, start, end)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Node {
    pos: Coords,
    cost: u32,
    prev: Option<Coords>,
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
    let (y, x) = pos;

    VECTORS
        .iter()
        .map(|(dy, dx)| (y + dy, x + dx))
        .filter(|(new_y, new_x)| {
            *new_x >= 0
                && *new_x < grid[0].len() as i32 - 1
                && *new_y >= 0
                && *new_y < grid.len() as i32 - 1
                && grid[*new_y as usize][*new_x as usize] != '#'
        })
        .collect()
}

fn find_path(grid: &Grid, start: Coords, end: Coords) -> Option<(u32, Vec<Node>)> {
    let mut prio_q = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut came_from = HashMap::new();

    let start_node = Node {
        pos: start,
        cost: 0,
        prev: None,
    };

    prio_q.push(start_node);
    distances.insert(start, 0);

    while let Some(current) = prio_q.pop() {
        if current.pos == end {
            let mut path = Vec::new();
            let mut current_node = current;

            path.push(current_node);
            while let Some(prev_pos) = current_node.prev {
                if let Some(&prev_node) = came_from.get(&prev_pos) {
                    path.push(prev_node);
                    current_node = prev_node;
                } else {
                    break;
                }
            }

            path.reverse();
            return Some((current.cost, path));
        }

        if let Some(&best) = distances.get(&current.pos) {
            if current.cost > best {
                continue;
            }
        }

        for neighbour in get_neighbors(current.pos, grid) {
            let next_cost = current.cost + 1;

            if !distances.contains_key(&neighbour)
                || next_cost < *distances.get(&neighbour).unwrap()
            {
                let next_node = Node {
                    pos: neighbour,
                    cost: next_cost,
                    prev: Some(current.pos),
                };

                distances.insert(neighbour, next_cost);
                came_from.insert(neighbour, next_node.clone());
                prio_q.push(next_node);
            }
        }
    }
    None
}

fn find_possible(
    (pos, curr_cost): (Coords, i32),
    path: &Vec<Node>,
    distance: usize,
    total_cost: u32,
) -> usize {
    let mut count = 0;
    for vec in VECTORS {
        let (mut ny, mut nx) = pos;
        for i in 1..=distance {
            ny += vec.0;
            nx += vec.1;

            if let Some(path) = path.iter().find(|n| n.pos == (ny, nx)) {
                if path.cost - curr_cost as u32 >= distance.try_into().unwrap() {
                    let diff = curr_cost + i as i32;
                    let diff_to_end = total_cost - path.cost;
                    let route_cost = diff + diff_to_end as i32;

                    if route_cost <= (total_cost - 100).try_into().unwrap() {
                        count += 1
                    }
                }
            }
        }
    }
    count
}

fn part1((grid, start, end): TInput) -> usize {
    let (cost, path) = find_path(&grid, start, end).unwrap();
    path.iter()
        .map(|n| find_possible((n.pos, n.cost as i32), &path, 2, cost))
        .sum()
}

fn part2() -> usize {
    0
}
