use std::{collections::BTreeMap, fs};

fn main() {
    let input = parse_input("input.txt");
    println!("Part 1 - {}", part1(input.clone()));
    println!("Part 2 - {}", part2("input.txt"));
}

#[derive(Clone, Debug)]
struct Chunk {
    size: u64,
    free: u64,
    id: u64,
}

fn parse_input(path: &str) -> Vec<Chunk> {
    let input = fs::read_to_string(path)
        .expect("Failed to read input...")
        .chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let test = input
        .chunks(2)
        .enumerate()
        .map(|(idx, c)| {
            let mut free = 0;
            if c.len() > 1 {
                free = c[1]
            }
            Chunk {
                size: c[0],
                free,
                id: idx as u64,
            }
        })
        .collect::<Vec<_>>();
    test
}

fn part1(mut input: Vec<Chunk>) -> u64 {
    let mut order: Vec<u64> = Vec::new();

    for _ in 0..input[0].size {
        order.push(input[0].id);
    }

    loop {
        let len = input.len();
        if len <= 2 {
            for _ in 0..input[len - 1].size {
                order.push(input[len - 1].id);
            }
            break;
        }

        while input[0].free > 0 && input[len - 1].size > 0 {
            order.push(input[len - 1].id);
            input[len - 1].size -= 1;
            input[0].free -= 1;
        }

        if input[len - 1].size == 0 {
            input.pop();
        }

        if input[0].free == 0 {
            for _ in 0..input[1].size {
                order.push(input[1].id);
            }
            input.remove(0);
        }
    }

    order
        .iter()
        .enumerate()
        .map(|(i, id)| (i as u64 * *id as u64) as u64)
        .collect::<Vec<_>>()
        .into_iter()
        .sum()
}

fn part2(path: &str) -> u64 {
    let input = fs::read_to_string(path).expect("Failed to read input...");
    let mut files = Vec::new();
    let mut spaces = Vec::new();

    let mut id: u64 = 0;
    for (idx, c) in input.chars().enumerate() {
        let len = c.to_string().parse::<u64>().unwrap();
        if idx % 2 == 0 {
            files.push((id, len));
        } else {
            spaces.push((id, len));
        }
        id += len;
        continue;
    }

    let mut space_ids: BTreeMap<u64, u64> = BTreeMap::from_iter(spaces);

    let mut result = 0;

    for (file_id, &(id, size)) in files.iter().enumerate().rev() {
        let free_space = space_ids
            .iter()
            .take_while(|&(&block_id, _)| block_id < id)
            .find(|&(_, &s)| s >= size);

        let mut new_id = id;
        match free_space {
            Some((&fs_idx, &free_space_size)) => {
                new_id = fs_idx;
                space_ids.remove(&fs_idx);
                if free_space_size > size {
                    space_ids.insert(fs_idx + size, free_space_size - size);
                }
            }
            _ => {}
        }

        result += (file_id as u64) * (new_id * 2 + size - 1) * size / 2;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(TEST_INPUT_PATH)), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_PATH), 2858);
    }
}
