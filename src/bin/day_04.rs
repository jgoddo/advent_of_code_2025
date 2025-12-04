use std::{collections::HashSet, fs};

const TEST: bool = true;

struct Grid {
    lift_locs: HashSet<(i32, i32)>,
    paper_locs: HashSet<(i32, i32)>,
    height: i32,
    width: i32,
}

impl Grid {
    fn new(grid: String) -> Grid {
        let height: i32 = grid.lines().count().try_into().unwrap();
        let width: i32 = grid.lines().next().unwrap().len().try_into().unwrap();
        let mut lift_locs: HashSet<(i32, i32)> = HashSet::new();
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
                    'x' => {
                        lift_locs.insert((y as i32, x as i32));
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
            lift_locs,
        };
    }

    fn get_accessable_locs(self) -> Vec<(i32, i32)> {
        let mut accessable_locs = Vec::new();
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
        //should only iterato over paper_locs!
        for y in 0..self.height {
            for x in 0..self.width {
                if self.paper_locs.contains(&(y, x)) {
                    let mut neighbour_count = 0;
                    for (y_offset, x_offset) in offsets {
                        if self.paper_locs.contains(&(y + y_offset, x + x_offset)) {
                            neighbour_count += 1;
                        }
                    }
                    if neighbour_count < 4 {
                        accessable_locs.push((y, x));
                        println!("{}, {}", y, x);
                    }
                }
            }
        }
        return accessable_locs;
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

    let res1 = grid.get_accessable_locs().len();
    println!("part1: {}", res1);
}
