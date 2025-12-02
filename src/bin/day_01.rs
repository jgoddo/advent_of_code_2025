use std::{cmp::max, fs};

fn get_input() -> String {
    let path = std::path::Path::new("./inputs/01.txt");
    let content: String = fs::read_to_string(path).expect("Failed to read File.");
    return content;
}

fn main() {
    let input = get_input();

    let mut pos = 50;
    let mut res_1 = 0;

    for line in input.split('\n') {
        let (first, rest) = line.split_at(1);
        let mut offset: i32 = rest.parse().unwrap();

        offset = match first {
            "L" => -offset,
            "R" => offset,
            _ => panic!("unknown char"),
        };

        pos += offset;

        pos = ((pos % 100) + 100) % 100;

        if pos == 0 {
            res_1 += 1;
        }
    }

    println!("part1: {}", res_1);

    let mut pos = 50;
    let mut res_2 = 0;

    for line in input.split('\n') {
        let (first, rest) = line.split_at(1);
        let mut offset: i32 = rest.parse().unwrap();

        offset = match first {
            "L" => -offset,
            "R" => offset,
            _ => panic!("unknown char"),
        };

        pos += offset;

        if pos >= 100 || pos <= 0 {
            res_2 += max((offset / 100).abs(), 1);
        }

        pos = ((pos % 100) + 100) % 100;
    }
    println!("part2: {}", res_2);
}
