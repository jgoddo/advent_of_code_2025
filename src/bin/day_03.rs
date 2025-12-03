use std::fs;

const TEST: bool = false;

fn get_input() -> String {
    let path = match TEST {
        true => std::path::Path::new("./inputs/03_test.txt"),
        false => std::path::Path::new("./inputs/03.txt"),
    };
    let content: String = fs::read_to_string(path).expect("Failed to read File.");
    return content;
}

fn solve(line: &str, nchars: usize) -> String {
    let mut res: std::vec::Vec<char> = std::vec::Vec::new();

    let mut start = 0;

    for offset in 0..nchars {
        let (idx, val) = line[start..line.len() - (nchars - offset - 1)]
            .chars()
            .enumerate()
            .max_by_key(|&(idx, el)| (el, -(idx as i32)))
            .unwrap();
        start += idx + 1;
        res.push(val);
    }
    return res.iter().collect();
}

fn main() {
    let input = get_input();

    let mut res1 = 0;
    for line in input.split('\n') {
        let val: String = solve(line, 2);
        res1 += val.parse::<i32>().unwrap();
    }
    println!("part 1: {}", res1);

    let mut res2: i64 = 0;

    for line in input.split('\n') {
        let val2: String = solve(line, 12);
        res2 += val2.parse::<i64>().unwrap();
    }
    println!("part 2: {}", res2);
}
