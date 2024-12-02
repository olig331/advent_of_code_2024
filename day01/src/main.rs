use std::fs;

fn main() {
    let (mut left, mut right) = fs::read_to_string("input.txt")
        .expect("Failed to read input...")
        .lines()
        .map(|l| {
            let res: Vec<u32> = l
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            (res[0], res[1])
        })
        .collect::<(Vec<u32>, Vec<u32>)>();

    left.sort();
    right.sort();

    let (mut p1, mut p2) = (0, 0);
    left.iter().enumerate().for_each(|(i, val)| {
        p1 += val.abs_diff(right[i]);
        p2 += val * right.clone().into_iter().filter(|d| d == val).count() as u32;
    });

    println!("Part 1 - {}", p1);
    println!("Part 2 - {}", p2)
}
