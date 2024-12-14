use itertools::*;
use std::fs;
use std::fs::File;
use std::io::Write;

type Coords = (i64, i64);

fn main() {
    println!("Part 1 - {}", part1());
    println!("Part 2 - {}", part2());
}

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

#[derive(Debug)]
struct Robot {
    pos: Coords,
    vel: Coords,
}

impl Robot {
    fn move_robot(&mut self) {
        let (nx, ny) = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1);

        if nx < 0 {
            self.pos.0 = WIDTH - nx.abs_diff(0) as i64
        } else if nx >= WIDTH {
            self.pos.0 = 0 + nx.abs_diff(WIDTH) as i64
        } else {
            self.pos.0 = nx;
        }

        if ny < 0 {
            self.pos.1 = HEIGHT - ny.abs_diff(0) as i64
        } else if ny >= HEIGHT {
            self.pos.1 = 0 + ny.abs_diff(HEIGHT) as i64
        } else {
            self.pos.1 = ny;
        }
    }
}

fn parse_input(path: &str) -> Vec<Robot> {
    let robots = fs::read_to_string(path)
        .expect("Failed to read input...")
        .lines()
        .map(|ro| {
            let r = ro
                .split_ascii_whitespace()
                .map(|p| {
                    p.split("=")
                        .nth(1)
                        .and_then(|x| {
                            x.split(",")
                                .map(|d| d.parse::<i64>().unwrap())
                                .collect_tuple::<Coords>()
                        })
                        .unwrap()
                })
                .collect_tuple::<(Coords, Coords)>()
                .unwrap();
            Robot { pos: r.0, vel: r.1 }
        })
        .collect::<Vec<_>>();
    robots
}

fn count_touching(robots: &Vec<Robot>) -> usize {
    let mut touching_count = 0;
    for i in 0..robots.len() {
        for j in (i + 1)..robots.len() {
            let (x1, y1) = robots[i].pos;
            let (x2, y2) = robots[j].pos;
            if (x1 - x2).abs() <= 1 && (y1 - y2).abs() <= 1 {
                touching_count += 1;
            }
        }
    }
    touching_count
}

fn part1() -> i64 {
    let mut robots = parse_input("input.txt");
    let mut quads = [0; 4];
    let q_x = (WIDTH - 1) / 2;
    let q_y = (HEIGHT - 1) / 2;

    for _ in 1..=100 {
        for r in &mut robots {
            r.move_robot();
        }
    }

    for r in robots {
        if r.pos.0 < q_x && r.pos.1 < q_y {
            quads[0] += 1
        }

        if r.pos.0 > q_x && r.pos.1 < q_y {
            quads[1] += 1;
        }

        if r.pos.0 < q_x && r.pos.1 > q_y {
            quads[2] += 1;
        }

        if r.pos.0 > q_x && r.pos.1 > q_y {
            quads[3] += 1;
        }
    }
    quads.iter().product()
}

fn part2() -> u32 {
    let mut robots = parse_input("input.txt");
    let len = robots.len();
    let mut iteration = 1;
    let mut file = File::create("pic.txt").expect("Failed to create file...");

    loop {
        for r in &mut robots {
            r.move_robot();
        }
        let count = count_touching(&robots);
        if count > len {
            let mut grid = [['.'; WIDTH as usize]; HEIGHT as usize];
            for r in robots {
                grid[r.pos.1 as usize][r.pos.0 as usize] = '#';
            }

            for row in grid {
                writeln!(file, "{}", row.map(|r| r.to_string()).join(""))
                    .expect("Failed to write...");
            }

            break;
        }
        iteration += 1;
    }

    iteration
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
