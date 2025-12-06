use std::fs;

const TEST: bool = false;

fn get_input() -> String {
    let path = match TEST {
        true => std::path::Path::new("./inputs/06_test.txt"),
        false => std::path::Path::new("./inputs/06.txt"),
    };
    let content: String = fs::read_to_string(path).expect("Failed to read File.");
    return content;
}

fn part1(input: &String) -> u64 {
    let mut instructions = Vec::new();
    let mut vals: Vec<Vec<u64>> = Vec::new();
    let n_lines = input.lines().count();

    for _ in 0..n_lines - 1 {
        vals.push(Vec::new());
    }

    for (idx, line) in input.lines().enumerate() {
        for el in line.split_whitespace() {
            if idx == n_lines - 1 {
                instructions.push(el);
            } else {
                vals[idx].push(el.parse().unwrap());
            }
        }
    }

    let mut res1 = 0;
    for (idx, instr) in instructions.iter().enumerate() {
        match *instr {
            "*" => {
                res1 += vals.iter().map(|el| el[idx]).product::<u64>();
            }
            "+" => {
                res1 += vals.iter().map(|el| el[idx]).sum::<u64>();
            }
            _ => {
                panic!("invalid instruction")
            }
        }
    }
    return res1;
}

fn part2(input: String) -> u64 {
    let mut res2 = 0;
    let mut instructions = Vec::new();
    let mut vals: Vec<Vec<char>> = Vec::new();
    let n_lines = input.lines().count();

    for _ in 0..n_lines - 1 {
        vals.push(Vec::new());
    }

    for (idx, line) in input.lines().enumerate() {
        for el in line.chars().rev() {
            if idx == n_lines - 1 {
                instructions.push(el);
            } else {
                vals[idx].push(el);
            }
        }
    }

    let mut numbers: Vec<u64> = Vec::new();
    let mut curr_number = String::new();
    let mut did_compute = false;
    for idx in 0..vals[0].len() {
        if did_compute {
            numbers.clear();
            did_compute = false;
            continue;
        }
        curr_number.clear();
        for n in 0..n_lines - 1 {
            curr_number.push(vals[n][idx]);
        }
        numbers.push(curr_number.trim().parse().unwrap());

        match instructions[idx] {
            '*' => {
                res2 += numbers.iter().product::<u64>();
                did_compute = true;
            }
            '+' => {
                res2 += numbers.iter().sum::<u64>();
                did_compute = true;
            }
            ' ' => { //pass
            }
            _ => {
                panic!("unknown Instruction")
            }
        }
    }
    return res2;
}

fn main() {
    let input = get_input();
    let res1 = part1(&input);
    println!("part1: {}", res1);

    let res2 = part2(input);
    println!("part2: {}", res2);
}
