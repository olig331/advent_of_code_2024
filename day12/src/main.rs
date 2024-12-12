use std::{collections::HashSet, fs, time::Instant};

type Coords = (i32, i32);

fn main() {
    let mut bench = Instant::now();
    println!(
        "Part 1 - {} | Took {:?}",
        part1(parse_input("input.txt")),
        bench.elapsed()
    );
    bench = Instant::now();
    println!(
        "Part 2 - {} | Took {:?}",
        part2(parse_input("input.txt")),
        bench.elapsed()
    );
}

fn parse_input(path: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(path)
        .expect("Failed to read input...")
        .lines()
        .map(|row| row.chars().collect())
        .collect();
    input
}

const VECTORS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Clone, Debug)]
struct Details {
    plant: char,
    positions: HashSet<Coords>,
}

fn recursion(input: &Vec<Vec<char>>, plant: &mut Details, pos: Coords, done: &mut HashSet<Coords>) {
    let (y, x) = pos;

    for (vy, vx) in VECTORS {
        let (ny, nx) = ((y + vy), (x + vx));
        if ny >= 0 && ny < input.len() as i32 && nx >= 0 && nx < input[y as usize].len() as i32 {
            if input[ny as usize][nx as usize] == plant.plant && !done.contains(&(ny, nx)) {
                done.insert((ny as i32, nx as i32));
                plant.positions.insert((ny as i32, nx as i32));
                recursion(input, plant, (ny, nx), done);
            }
        }
    }
}

fn gen_map(input: &Vec<Vec<char>>) -> Vec<Details> {
    let mut done: HashSet<Coords> = HashSet::new();
    let mut result: Vec<Details> = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let mut plant = Details {
                plant: input[y][x],
                positions: HashSet::new(),
            };

            if done.get(&(y as i32, x as i32)).is_none() {
                done.insert((y as i32, x as i32));
                plant
                    .positions
                    .insert((y.try_into().unwrap(), x.try_into().unwrap()));

                recursion(&input, &mut plant, (y as i32, x as i32), &mut done);
                result.push(plant);
            }
        }
    }
    result
}

fn part1(input: Vec<Vec<char>>) -> u64 {
    let mut sum = 0 as u64;
    let result = gen_map(&input);

    for item in result {
        let mut perimeter = 0;
        for (y, x) in &item.positions {
            for (vy, vx) in VECTORS {
                if !item.positions.contains(&(y - vy, x - vx)) {
                    perimeter += 1;
                }
            }
        }
        sum += (item.positions.len() * perimeter) as u64;
    }
    sum
}

pub fn part2(input: Vec<Vec<char>>) -> usize {
    let mut total = 0;
    let result = gen_map(&input);

    for p in result {
        let mut edges = HashSet::new();
        for &(y, x) in &p.positions {
            for (idx, (vy, vx)) in VECTORS.iter().enumerate() {
                let (ny, nx) = ((y as i32 + vy) as usize, (x as i32 + vx) as usize);

                if ny >= input.len() || nx >= input[0].len() || input[ny][nx] != p.plant {
                    edges.insert((y, x, idx));
                }
            }
        }

        let mut sides = 0;
        let mut checked = HashSet::new();

        for &(y, x, dir) in &edges {
            if checked.contains(&(y, x, dir)) {
                continue;
            }

            sides += 1;

            let oppos = match dir {
                0 | 2 => [(1, 0), (-1, 0)],
                1 | 3 => [(0, 1), (0, -1)],
                _ => unreachable!(),
            };

            for &(oy, ox) in &oppos {
                let (mut ny, mut nx) = (y as i32, x as i32);

                loop {
                    ny += oy;
                    nx += ox;

                    if ny < 0
                        || ny >= input.len() as i32
                        || nx < 0
                        || nx >= input[y as usize].len() as i32
                    {
                        break;
                    }

                    let curr_pos = (ny as i32, nx as i32, dir);
                    if !edges.contains(&curr_pos) {
                        break;
                    }

                    checked.insert(curr_pos);
                }
            }
        }
        total += p.positions.len() * sides;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(TEST_INPUT_PATH)), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(TEST_INPUT_PATH)), 1206);
    }
}
