use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, path,
    sync::Mutex,
};

const TEST: bool = false;

fn get_input() -> String {
    let path = match TEST {
        true => std::path::Path::new("./inputs/07_test.txt"),
        false => std::path::Path::new("./inputs/07.txt"),
    };
    let content: String = fs::read_to_string(path).expect("Failed to read File.");
    return content;
}

fn main() {
    let input = get_input();

    let mut beam_positions = HashSet::new();
    let mut splitter_positions = HashSet::new();
    let mut start_pos = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, val) in line.chars().enumerate() {
            match val {
                'S' => {
                    beam_positions.insert((y, x));
                    start_pos = (y, x);
                }
                '^' => {
                    splitter_positions.insert((y, x));
                }
                '.' => { /*  */ }
                _ => {
                    panic!("unknown char!")
                }
            }
        }
    }

    let height = input.lines().count();
    let mut res1 = 0;
    let mut new_positions = HashSet::new();

    for step in 0..height + 1 {
        new_positions.clear();
        for beam in beam_positions.iter() {
            let new_pos = (beam.0 + 1, beam.1);
            if splitter_positions.contains(&new_pos) {
                res1 += 1;
                new_positions.insert((beam.0 + 1, beam.1 + 1));
                new_positions.insert((beam.0 + 1, beam.1 - 1));
            } else {
                new_positions.insert(new_pos);
            }
        }
        std::mem::swap(&mut beam_positions, &mut new_positions);
    }

    println!("part1: {}", res1);

    let res2 = get_num_paths(start_pos, &splitter_positions, height, 0);

    println!("part2: {}", res2);
}

static paths_buf: std::sync::LazyLock<Mutex<HashMap<(usize, usize), usize>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

fn get_num_paths(
    beam: (usize, usize),
    splitter_positions: &HashSet<(usize, usize)>,
    height: usize,
    val: usize,
) -> usize {
    if beam.0 == height {
        return val + 1;
    }

    let new_pos = (beam.0 + 1, beam.1);

    if let Some(val) = paths_buf.lock().unwrap().get(&new_pos) {
        return *val;
    }
    let mut res = 0;
    if splitter_positions.contains(&new_pos) {
        let res_left = get_num_paths((beam.0 + 1, beam.1 + 1), splitter_positions, height, val);
        paths_buf
            .lock()
            .unwrap()
            .insert((beam.0 + 1, beam.1 + 1), res_left);
        let res_right = get_num_paths((beam.0 + 1, beam.1 - 1), splitter_positions, height, val);
        paths_buf
            .lock()
            .unwrap()
            .insert((beam.0 + 1, beam.1 - 1), res_right);
        res += res_left + res_right;
    } else {
        let res_down = get_num_paths(new_pos, splitter_positions, height, val);
        paths_buf.lock().unwrap().insert(new_pos, res_down);
        res += res_down;
    }
    return res;
}
