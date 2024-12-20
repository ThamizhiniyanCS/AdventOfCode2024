use regex::Regex;
use std::time::{Duration, Instant};
use std::{fs, isize};

fn calculate_position(
    width: &isize,
    height: &isize,
    position: &(isize, isize),
    velocity: &(isize, isize),
    seconds: isize,
) -> (isize, isize) {
    let x = ((position.0 + (velocity.0 * seconds)) % width + width) % width;
    let y = ((position.1 + (velocity.1 * seconds)) % height + height) % height;

    (x, y)
}

fn calculate_safety_score(
    positions: Vec<(isize, isize)>,
    x_middle: &isize,
    y_middle: &isize,
) -> isize {
    let mut quadrant_1: isize = 0;
    let mut quadrant_2: isize = 0;
    let mut quadrant_3: isize = 0;
    let mut quadrant_4: isize = 0;

    for (x, y) in positions {
        if x != *x_middle && y != *y_middle {
            if x < *x_middle && y < *y_middle {
                quadrant_1 += 1
            }
            if x > *x_middle && y < *y_middle {
                quadrant_2 += 1
            }
            if x < *x_middle && y > *y_middle {
                quadrant_3 += 1
            }
            if x > *x_middle && y > *y_middle {
                quadrant_4 += 1
            }
        }
    }

    quadrant_1 * quadrant_2 * quadrant_3 * quadrant_4
}

fn part_1(
    robots_list: &Vec<((isize, isize), (isize, isize))>,
    width: &isize,
    height: &isize,
    x_middle: &isize,
    y_middle: &isize,
) -> isize {
    let mut positions: Vec<(isize, isize)> = Vec::new();

    for (position, velocity) in robots_list {
        positions.push(calculate_position(width, height, position, velocity, 100))
    }

    calculate_safety_score(positions, x_middle, y_middle)
}

fn part_2(
    robots_list: &Vec<((isize, isize), (isize, isize))>,
    width: &isize,
    height: &isize,
    x_middle: &isize,
    y_middle: &isize,
) -> isize {
    let mut safety_scores: Vec<isize> = Vec::new();

    for second in 0..(width * height) {
        let mut positions: Vec<(isize, isize)> = Vec::new();

        for (position, velocity) in robots_list {
            positions.push(calculate_position(
                width, height, position, velocity, second,
            ))
        }

        safety_scores.push(calculate_safety_score(positions, x_middle, y_middle));
    }

    safety_scores
        .iter()
        .enumerate()
        .min_by_key(|&(_, &score)| score)
        .map(|(i, _)| i as isize)
        .unwrap()
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file: String = fs::read_to_string("../inputs/Day 14: Restroom Redoubt/input.txt")
        .expect("Expecting a valid file.");

    let mut robots_list: Vec<((isize, isize), (isize, isize))> = Vec::new();

    let regex_pattern: Regex =
        Regex::new(r"p=(\d{1,3}),(\d{1,3})\sv=(-?\d{1,2}),(-?\d{1,2})").unwrap();

    for line in file.lines() {
        let position_velocity = regex_pattern.captures(line).unwrap();

        let position: (isize, isize) = (
            position_velocity
                .get(1)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
            position_velocity
                .get(2)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
        );

        let velocity: (isize, isize) = (
            position_velocity
                .get(3)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
            position_velocity
                .get(4)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
        );

        robots_list.push((position, velocity));
    }

    let width: isize = 101;
    let height: isize = 103;
    let x_middle: isize = width / 2;
    let y_middle: isize = height / 2;

    println!();
    println!("[+] Part One Solution");
    println!(
        ">>> Safety factor after 100 seconds: {}",
        part_1(&robots_list, &width, &height, &x_middle, &y_middle)
    );
    println!();
    println!("[+] Part Two Solution");
    println!(
        ">>> The fewest number of seconds that must elapse for the robots to display the Easter egg: {}",
        part_2(&robots_list, &width, &height, &x_middle, &y_middle)
    );

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "\nTotal Time Taken: {} seconds.",
        total_time_taken.as_secs_f64()
    );
}
