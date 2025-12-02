use std::fs;

fn get_input() -> String {
    let path = std::path::Path::new("./inputs/02.txt");
    let content: String = fs::read_to_string(path).expect("Failed to read File.");
    return content;
}

fn main() {
    let input = get_input();

    let mut res1 = 0;
    for range in input.split(',') {
        let split_idx = range.find('-').unwrap();
        let start: i64 = range[..split_idx].parse().unwrap();
        let end: i64 = range[split_idx + 1..].parse().unwrap();

        for check_val in start..end + 1 {
            let check_str = check_val.to_string();

            //every uneven length is valid
            if check_str.len() % 2 == 0 {
                let offset = check_str.len() / 2;
                if check_str[..offset] == check_str[offset..] {
                    res1 += check_val;
                }
            }
        }
    }
    println!("part1: {}", res1);

    let mut res2 = 0;
    for range in input.split(',') {
        let split_idx = range.find('-').unwrap();
        let start: i64 = range[..split_idx].parse().unwrap();
        let end: i64 = range[split_idx + 1..].parse().unwrap();

        for check_val in start..end + 1 {
            let check_str = check_val.to_string();

            for seq_len in 1..(check_str.len() / 2) + 1 {
                if check_str.len() % seq_len != 0 {
                    continue;
                }

                let seq = &check_str[..seq_len];
                let mut is_valid = true;
                for start_idx in (seq_len..check_str.len() - seq_len + 1).step_by(seq_len) {
                    if check_str[start_idx..start_idx + seq_len] != *seq {
                        is_valid = false;
                        break;
                    }
                }
                if is_valid {
                    res2 += check_val;
                    break;
                }
            }
        }
    }
    println!("part2: {}", res2);
}
