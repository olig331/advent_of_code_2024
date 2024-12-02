fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let (p1, p2) = (part1(&input), part2(&input));
    println!("Part 1 - {}", p1);
    println!("Part 2 - {}", p2);
}

fn is_valid(input: &Vec<u32>) -> bool {
    if !input.is_sorted() && !input.iter().rev().is_sorted() {
        return false;
    }

    if input.windows(2).all(|x| {
        let n = x[0].abs_diff(x[1]);
        n >= 1 && n <= 3
    }) {
        return true;
    };

    false
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let mut result = 0;
    for row in input.clone() {
        if is_valid(&row) {
            result += 1;
        }
    }
    result
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut result = 0;

    for row in input {
        if is_valid(&row) {
            result += 1;
            continue;
        }

        for i in 0..row.len() {
            let per_mutation = row
                .iter()
                .enumerate()
                .filter(|(y, _)| *y != i)
                .map(|(_, &x)| x)
                .collect::<Vec<_>>();

            if is_valid(&per_mutation) {
                result += 1;
                break;
            }
        }
    }
    result
}
