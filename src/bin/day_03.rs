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

fn main() {
    let input = get_input();

    let mut res1 = 0;
    for line in input.split('\n') {
        let (first_idx, first_val) = line[..line.len() - 1]
            .chars()
            .enumerate()
            .max_by_key(|&(idx, el)| (el, -(idx as i32)))
            .unwrap();
        let (second_idx, second_val) = line[first_idx + 1..]
            .chars()
            .enumerate()
            .max_by_key(|&(idx, el)| (el, -(idx as i32)))
            .unwrap();

        if TEST {
            println!("{} -> {}{}", line, first_val, second_val)
        }
        let val: String = [first_val, second_val].iter().collect();
        res1 += val.parse::<i32>().unwrap();
    }
    println!("part 1: {}", res1); // 16902 too low
}
