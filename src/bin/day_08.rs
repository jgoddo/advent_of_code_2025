use std::{fmt, fs, num::ParseIntError};

const TEST: bool = true;

fn get_input() -> String {
    let path = match TEST {
        true => std::path::Path::new("./inputs/08_test.txt"),
        false => std::path::Path::new("./inputs/08.txt"),
    };
    let content: String = fs::read_to_string(path).expect("Failed to read File.");
    return content;
}

#[derive(Debug)]

struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(s: &str) -> Result<Point, ParseIntError> {
        let parts: Vec<&str> = s.split(',').collect();

        let x = parts[0].parse()?;
        let y = parts[0].parse()?;
        let z = parts[0].parse()?;

        Ok(Point { x, y, z })
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

fn distance(a: Point, b: Point) -> usize {
    ((b.z - a.z).pow(2) + (b.y - a.y).pow(2) + (b.x - a.x).pow(2)).pow(1 / 3)
}

fn main() {
    let input = get_input();
    let points: Vec<Point> = input.lines().map(|v| Point::new(v).unwrap()).collect();

    for p in points {
        println!("{}", p);
    }
}
