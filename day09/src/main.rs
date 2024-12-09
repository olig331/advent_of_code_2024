use std::fs;

fn main() {
    let input = parse_input("test_input.txt");
    println!("Part 1 - {}", part1(input.clone()));
    println!("Part 2 - {}", part2(input.clone(), "test_input.txt"));
}

#[derive(Clone, Debug)]
struct Chunk {
    size: i64,
    original_free: i64,
    free: i64,
    id: i64,
    moved: bool,
}

fn parse_input(path: &str) -> Vec<Chunk> {
    let input = fs::read_to_string(path)
        .expect("Failed to read input")
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
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
                original_free: free,
                id: idx as i64,
                moved: false,
            }
        })
        .collect::<Vec<_>>();
    test
}

fn part1(mut input: Vec<Chunk>) -> i64 {
    let mut order: Vec<i64> = Vec::new();

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
        .map(|(i, id)| (i as i64 * *id as i64) as i64)
        .collect::<Vec<_>>()
        .into_iter()
        .sum()
}

fn part2(mut input: Vec<Chunk>, path: &str) -> u64 {
    let mut order = fs::read_to_string(path)
        .expect("Failed to read input")
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .flat_map(|(y, s)| {
            let mut vec = Vec::new();
            for _ in 0..s[0].parse().unwrap() {
                vec.push(y.to_string());
            }
            if s.len() > 1 {
                for _ in 0..s[1].parse().unwrap() {
                    vec.push(".".to_owned());
                }
            }
            vec
        })
        .collect::<Vec<_>>();

    for c in &order {
        print!("{}", c)
    }
    println!("");
    println!("{}", order.len());

    'outer: loop {
        let mut len = input.len();
        if len <= 2 {
            break;
        }

        if input[0].free == 0 {
            input.remove(0);
        }

        len = input.len();
        let mut passed = false;

        for i in (1..len).rev() {
            if input[i].moved {
                continue;
            }

            if input[i].size <= input[0].free {
                // find the last index of id
                // Get last index + free
                // find the difference between original free and free
                // swap at last index of id + diff

                let last_index = order.iter().rposition(|x| *x == input[0].id.to_string());
                let diff = (input[0].original_free - input[0].free) as usize + 1;

                let swap_last_index = order.iter().rposition(|x| *x == input[i].id.to_string());

                if last_index.is_none() || swap_last_index.is_none() {
                    continue;
                }

                println!("Swapping {} into fs {}", input[i].id, input[0].id);
                // println!(
                //     "{} {} {}",
                //     last_index.unwrap(),
                //     diff,
                //     swap_last_index.unwrap()
                // );

                println!("");

                for i in 0..input[i].size {
                    println!(
                        "{} {} {}",
                        last_index.unwrap() + diff + i as usize,
                        diff,
                        swap_last_index.unwrap() - i as usize
                    );
                    order.swap(
                        (last_index.unwrap() + diff) + i as usize,
                        swap_last_index.unwrap() - i as usize,
                    );
                }

                for c in &order {
                    print!("{}", c)
                }
                println!("");
                // if item.moved && item.free > 0 {
                //             for _ in 0..item.free {
                //                 let index = order.iter().rposition(|x| *x == item.id);
                //                 if index.is_some() {
                //                     order.insert(index.unwrap() + 1, 0);
                //                 }
                //             }
                //         }
                //         if item.moved && item.free > 0 {
                //             for _ in 0..=item.size {
                //                 let index = order.iter().position(|x| *x == item.id + 1);
                //                 if index.is_some() {
                //                     order.insert(index.unwrap(), 0);
                //                 }
                //             }
                //         }
                // }

                input[0].free -= input[i].size;
                input[i].moved = true;
                passed = true;

                break;
            }
        }

        if !passed {
            break 'outer;
        }
    }

    for c in &order {
        print!("{}", c)
    }
    0
}

// fn part2(input: &mut Vec<Chunk>) -> i64 {
//     let mut order: Vec<i64> = Vec::new();

//     for _ in 0..input[0].size {
//         order.push(input[0].id);
//     }

//     'outer: loop {
//         let mut len = input.len();
//         if len <= 2 {
//             break;
//         }

//         if input[0].free == 0 {
//             input.remove(0);
//         }

//         len = input.len();
//         let mut passed = false;

//         for i in (1..len).rev() {
//             if input[i].moved {
//                 continue;
//             }

//             if input[i].size <= input[0].free {
//                 let print_next = input[i].size == input[0].free;
//                 for _ in 0..input[i].size {
//                     order.push(input[i].id);
//                 }

//                 input[0].free -= input[i].size;
//                 input[i].moved = true;
//                 passed = true;

//                 if print_next && !input[1].moved {
//                     for _ in 0..input[1].size {
//                         order.push(input[1].id);
//                     }
//                 }
//                 break;
//             }
//         }

//         if !passed {
//             break 'outer;
//         }
//     }

//     for idx in 0..input.len() - 1 {
//         let chunk = &input[idx];
//         if chunk.moved {
//             continue;
//         }
//         for _ in 0..chunk.size {
//             order.push(chunk.id);
//         }

//         for _ in 0..chunk.free {
//             order.push(0);
//         }
//     }

//     for (i, item) in input.iter().enumerate() {
//         if i == 0 {
//             continue;
//         }

//         if item.moved && item.free > 0 {
//             for _ in 0..item.free {
//                 let index = order.iter().rposition(|x| *x == item.id);
//                 if index.is_some() {
//                     order.insert(index.unwrap() + 1, 0);
//                 }
//             }
//         }
//         if item.moved && item.free > 0 {
//             for _ in 0..=item.size {
//                 let index = order.iter().position(|x| *x == item.id + 1);
//                 if index.is_some() {
//                     order.insert(index.unwrap(), 0);
//                 }
//             }
//         }
//     }

//     order
//         .iter()
//         .enumerate()
//         .map(|(i, id)| (i as i64 * *id as i64) as i64)
//         .collect::<Vec<_>>()
//         .iter()
//         .sum()
// }

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
