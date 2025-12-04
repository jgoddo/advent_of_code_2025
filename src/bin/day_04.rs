use std::{collections::HashSet, fs};

const TEST: bool = false;

struct Grid {
    paper_locs: HashSet<(i32, i32)>,
    height: i32,
    width: i32,
}

impl Grid {
    fn new(grid: String) -> Grid {
        let height: i32 = grid.lines().count().try_into().unwrap();
        let width: i32 = grid.lines().next().unwrap().len().try_into().unwrap();
        let mut paper_locs: HashSet<(i32, i32)> = HashSet::new();

        for (y, line) in grid.lines().enumerate() {
            for (x, val) in line.chars().enumerate() {
                print!("{}", val);
                match val {
                    '.' => {
                        //free location dont care
                    }
                    '@' => {
                        //paper location
                        paper_locs.insert((y as i32, x as i32));
                    }

                    _ => {
                        panic!("invalid character found");
                    }
                }
            }
            println!();
        }
        return Grid {
            height,
            width,
            paper_locs,
        };
    }

    fn get_accessible_locs(self) -> Vec<(i32, i32)> {
        let mut accessible_locs = Vec::new();
        let offsets: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (y, x) in self.paper_locs.iter() {
            let mut neighbor_count = 0;
            for (y_offset, x_offset) in offsets {
                if self.paper_locs.contains(&(y + y_offset, x + x_offset)) {
                    neighbor_count += 1;
                }
            }
            if neighbor_count < 4 {
                accessible_locs.push((*y, *x));
                println!("{}, {}", y, x);
            }
        }
        return accessible_locs;
    }
}

fn get_input() -> Grid {
    let path = match TEST {
        true => std::path::Path::new("./inputs/04_test.txt"),
        false => std::path::Path::new("./inputs/04.txt"),
    };
    let content: String = fs::read_to_string(path).expect("Failed to read File.");
    return Grid::new(content);
}

fn main() {
    let grid = get_input();

    let res1 = grid.get_accessible_locs().len();
    println!("part1: {}", res1);
}
