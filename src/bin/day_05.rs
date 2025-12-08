use std::{
    cmp::{max, min},
    collections::HashSet,
    fs, vec,
};

const TEST: bool = false;

fn get_input() -> String {
    let path = match TEST {
        true => std::path::Path::new("./inputs/05_test.txt"),
        false => std::path::Path::new("./inputs/05.txt"),
    };
    let content: String = fs::read_to_string(path).expect("Failed to read File.");
    return content;
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, val: u64) -> bool {
        return val >= self.start && val <= self.end;
    }

    fn new(line: &str) -> Range {
        let split_idx = line.find("-").unwrap();

        let start: u64 = line[..split_idx].parse().expect("failed to parse");
        let end: u64 = line[split_idx + 1..].parse().expect("failed to parse");

        return Range { start, end };
    }

    fn can_merge(&self, other: &Range) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
    }

    fn merge(&mut self, other: &Range) {
        println!(
            "merging {}-{} & {}-{}",
            self.start, self.end, other.start, other.end
        );
        self.start = min(self.start, other.start);
        self.end = max(self.end, other.end);
    }
}

fn main() {
    let input = get_input();
    let mut ranges: Vec<Range> = Vec::new();
    let mut ranges_fin = false;
    let mut res1 = 0;
    for line in input.lines() {
        if line.is_empty() {
            // end of ranges block
            ranges_fin = true;
            continue;
        } else if !ranges_fin {
            ranges.push(Range::new(line));
        } else {
            for r in &ranges {
                if r.contains(line.parse().unwrap()) {
                    res1 += 1;
                    break;
                }
            }
        }
    }
    println!("part1: {}", res1);

    let mut res2 = 0;

    println!("part2: {}", res2);
}
