use itertools::enumerate;
use std::collections::HashSet;
use std::fs;
use std::time::{Duration, Instant};

fn get_next_position(
    no_of_rows: isize,
    no_of_columns: isize,
    i: isize,
    j: isize,
) -> [Option<(isize, isize)>; 4] {
    let up: Option<(isize, isize)> = if i - 1 < 0 { None } else { Some((i - 1, j)) };
    let down: Option<(isize, isize)> = if i + 1 < no_of_rows {
        Some((i + 1, j))
    } else {
        None
    };
    let left: Option<(isize, isize)> = if j - 1 < 0 { None } else { Some((i, j - 1)) };
    let right: Option<(isize, isize)> = if j + 1 < no_of_columns {
        Some((i, j + 1))
    } else {
        None
    };

    [up, down, left, right]
}

fn get_valid_positions(
    plot: &Vec<Vec<char>>,
    positions: [Option<(isize, isize)>; 4],
    value: char,
) -> HashSet<(isize, isize)> {
    let mut valid_positions: HashSet<(isize, isize)> = HashSet::new();

    for position in positions {
        if !position.is_none() {
            let (x, y) = position.unwrap();

            if plot[x as usize][y as usize] == value {
                valid_positions.insert((x, y));
            }
        }
    }

    valid_positions
}

fn enumerate_plant_region(
    plot: &Vec<Vec<char>>,
    no_of_rows: isize,
    no_of_columns: isize,
    plant: char,
    to_enumerate: (isize, isize),
    enumerated_positions: &mut HashSet<(isize, isize)>,
) {
    if enumerated_positions.contains(&to_enumerate) {
        return;
    }

    enumerated_positions.insert(to_enumerate);

    let (x, y) = to_enumerate;

    for position in get_valid_positions(
        plot,
        get_next_position(no_of_rows, no_of_columns, x, y),
        plant,
    ) {
        enumerate_plant_region(
            plot,
            no_of_rows,
            no_of_columns,
            plant,
            position,
            enumerated_positions,
        );
    }
}

fn calculate_perimeter(region: &HashSet<(isize, isize)>) -> usize {
    let mut sides: Vec<(isize, isize)> = Vec::new();

    for (x, y) in region {
        for position in [(x - 1, *y), (x + 1, *y), (*x, y - 1), (*x, y + 1)] {
            if !region.contains(&position) {
                sides.push(position);
            }
        }
    }

    sides.len()
}

fn calculate_sides(region: &HashSet<(isize, isize)>) -> usize {
    let mut up: HashSet<(isize, isize)> = HashSet::new();
    let mut down: HashSet<(isize, isize)> = HashSet::new();
    let mut left: HashSet<(isize, isize)> = HashSet::new();
    let mut right: HashSet<(isize, isize)> = HashSet::new();

    for (x, y) in region {
        if !region.contains(&(x - 1, *y)) {
            up.insert((*x, *y));
        }
        if !region.contains(&(x + 1, *y)) {
            down.insert((*x, *y));
        }
        if !region.contains(&(*x, y - 1)) {
            left.insert((*x, *y));
        }
        if !region.contains(&(*x, y + 1)) {
            right.insert((*x, *y));
        }
    }

    let mut count: usize = 0;

    for (x, y) in up {
        if left.contains(&(x, y)) {
            count += 1;
        }
        if right.contains(&(x, y)) {
            count += 1;
        }
        if right.contains(&(x - 1, y - 1)) && !left.contains(&(x, y)) {
            count += 1;
        }
        if left.contains(&(x - 1, y + 1)) && !right.contains(&(x, y)) {
            count += 1;
        }
    }

    for (x, y) in down {
        if left.contains(&(x, y)) {
            count += 1;
        }
        if right.contains(&(x, y)) {
            count += 1;
        }
        if right.contains(&(x + 1, y - 1)) && !left.contains(&(x, y)) {
            count += 1;
        }
        if left.contains(&(x + 1, y + 1)) && !right.contains(&(x, y)) {
            count += 1;
        }
    }

    count
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file: String = fs::read_to_string("../inputs/Day 12: Garden Groups/input.txt")
        .expect("Expecting a valid file.");

    let plot: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let no_of_rows: isize = plot.len() as isize;
    let no_of_columns: isize = plot[0].len() as isize;
    let mut explored_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut total_price_perimeter: usize = 0;
    let mut total_price_sides: usize = 0;

    for (i, row) in enumerate(&plot) {
        for (j, character) in enumerate(row) {
            if explored_positions.contains(&(i as isize, j as isize)) {
                continue;
            }

            let mut enumerated_positions: HashSet<(isize, isize)> = HashSet::new();

            enumerate_plant_region(
                &plot,
                no_of_rows,
                no_of_columns,
                *character,
                (i as isize, j as isize),
                &mut enumerated_positions,
            );

            let length_enumerated_positions: usize = enumerated_positions.len();

            for position in &enumerated_positions {
                explored_positions.insert(*position);
            }

            total_price_perimeter +=
                length_enumerated_positions * calculate_perimeter(&enumerated_positions);

            total_price_sides +=
                length_enumerated_positions * calculate_sides(&enumerated_positions);
        }
    }

    println!();
    println!("[+] Part One Solution");
    println!(
        ">>> Price of fencing based on perimeter: {}",
        total_price_perimeter
    );
    println!();
    println!("[+] Part Two Solution");
    println!(">>> Price of fencing based on sides: {}", total_price_sides);

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "\nTotal Time Taken: {} seconds.",
        total_time_taken.as_secs_f64()
    );
}
