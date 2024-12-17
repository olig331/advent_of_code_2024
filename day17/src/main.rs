use std::{fs, time::Instant};

fn main() {
    let bench = Instant::now();
    let (reg, ins) = parse_input("input.txt");
    println!("Part 1 - {} | Took {:?}", part1(reg, ins), bench.elapsed());
}

#[derive(Debug)]
struct Registers {
    a: i32,
    b: i32,
    c: i32,
}

fn parse_input(path: &str) -> (Registers, Vec<i8>) {
    let binding = fs::read_to_string(path).expect("Failed to read input...");
    let input = binding.split("\n\n").collect::<Vec<_>>();
    let registers = input[0]
        .lines()
        .into_iter()
        .map(|i| i.split(": ").nth(1).unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    (
        Registers {
            a: registers[0],
            b: registers[1],
            c: registers[2],
        },
        input[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|i| i.parse::<i8>().unwrap())
            .collect::<Vec<i8>>(),
    )
}

fn part1(mut regs: Registers, ins: Vec<i8>) -> String {
    let mut pointer = 0;
    let mut output = Vec::new();

    while pointer < ins.len() {
        let opc = &ins[pointer];
        let opr = match &ins[pointer + 1] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => regs.a,
            5 => regs.b,
            6 => regs.c,
            _ => unreachable!(),
        };

        match opc {
            0 => regs.a = regs.a / 2_i32.pow(opr.try_into().unwrap()),
            1 => regs.b = regs.b ^ ins[pointer + 1] as i32,
            2 => regs.b = opr as i32 % 8,
            3 => {
                if regs.a != 0 {
                    pointer = opr as usize;
                    continue;
                }
            }
            4 => regs.b = regs.b ^ regs.c,
            5 => output.push((opr % 8).to_string()),
            6 => regs.b = regs.a / 2_i32.pow(opr.try_into().unwrap()),
            7 => regs.c = regs.a / 2_i32.pow(opr.try_into().unwrap()),
            _ => unreachable!(),
        };
        pointer += 2;
    }
    output.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_PATH: &str = "test_input.txt";

    #[test]
    fn test_part1() {
        let (reg, ins) = parse_input(TEST_INPUT_PATH);
        assert_eq!(part1(reg, ins), "4,6,3,5,6,3,5,2,1,0".to_owned())
    }

    // #[test]
    // fn test_part2() {
    //     todo!()
    // }
}
