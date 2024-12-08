use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::{Duration, Instant};

use itertools::{enumerate, Itertools};

fn is_valid(elements: (isize, isize), min: &isize, max: &isize) -> bool {
    if !(elements.0 >= *min && elements.0 < *max) {
        return false;
    }
    if !(elements.1 >= *min && elements.1 < *max) {
        return false;
    }

    true
}

fn part_1(
    no_of_rows: &isize,
    no_of_columns: &isize,
    antenna_locations: &HashMap<char, Vec<Vec<isize>>>,
) {
    let mut antinode_locations: HashSet<(isize, isize)> = HashSet::new();

    for location in antenna_locations.values() {
        for pair in location.iter().combinations(2) {
            let pair_0_x: isize = pair[0][0];
            let pair_0_y: isize = pair[0][1];
            let pair_1_x: isize = pair[1][0];
            let pair_1_y: isize = pair[1][1];

            let x_diff: isize = pair_1_x - pair_0_x;
            let y_diff: isize = pair_1_y - pair_0_y;

            let pair_0_antinode: (isize, isize) = (pair_0_x - x_diff, pair_0_y - y_diff);
            let pair_1_antinode: (isize, isize) = (pair_1_x + x_diff, pair_1_y + y_diff);

            if is_valid(pair_0_antinode, &0, no_of_rows) {
                antinode_locations.insert(pair_0_antinode);
            }
            if is_valid(pair_1_antinode, &0, no_of_columns) {
                antinode_locations.insert(pair_1_antinode);
            }
        }
    }

    println!("[+] Part One Solution");
    println!(">>> Total Anitnodes: {}", antinode_locations.len());
}

fn part_2(
    no_of_rows: &isize,
    no_of_columns: &isize,
    antenna_locations: &HashMap<char, Vec<Vec<isize>>>,
) {
    let mut antinode_locations: HashSet<(isize, isize)> = HashSet::new();

    for location in antenna_locations.values() {
        for pair in location.iter().combinations(2) {
            antinode_locations.insert((pair[0][0], pair[0][1]));
            antinode_locations.insert((pair[1][0], pair[1][1]));

            let pair_0_x: isize = pair[0][0];
            let pair_0_y: isize = pair[0][1];
            let pair_1_x: isize = pair[1][0];
            let pair_1_y: isize = pair[1][1];

            let x_diff: isize = pair_1_x - pair_0_x;
            let y_diff: isize = pair_1_y - pair_0_y;

            let mut pair_0_break: bool = false;
            let mut pair_0_antinode: (isize, isize) = (pair_0_x - x_diff, pair_0_y - y_diff);

            if is_valid(pair_0_antinode, &0, no_of_rows) {
                antinode_locations.insert(pair_0_antinode);
            }

            while pair_0_break == false {
                pair_0_antinode = (pair_0_antinode.0 - x_diff, pair_0_antinode.1 - y_diff);

                if is_valid(pair_0_antinode, &0, no_of_rows) {
                    antinode_locations.insert(pair_0_antinode);
                } else {
                    pair_0_break = true;
                }
            }

            let mut pair_1_break: bool = false;
            let mut pair_1_antinode: (isize, isize) = (pair_1_x + x_diff, pair_1_y + y_diff);

            if is_valid(pair_1_antinode, &0, no_of_columns) {
                antinode_locations.insert(pair_1_antinode);
            }

            while pair_1_break == false {
                pair_1_antinode = (pair_1_antinode.0 + x_diff, pair_1_antinode.1 + y_diff);

                if is_valid(pair_1_antinode, &0, no_of_rows) {
                    antinode_locations.insert(pair_1_antinode);
                } else {
                    pair_1_break = true;
                }
            }
        }
    }

    println!("[+] Part Two Solution");
    println!(">>> Total Anitnodes: {}", antinode_locations.len());
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file = fs::read_to_string("../inputs/Day 8: Resonant Collinearity/input.txt")
        .expect("Expecting a valid file.");

    let mut map: Vec<Vec<char>> = Vec::new();

    let mut antenna_locations: HashMap<char, Vec<Vec<isize>>> = HashMap::new();

    for (i, line) in enumerate(file.lines().map(String::from)) {
        let mut characters: Vec<char> = Vec::new();

        for (j, character) in enumerate(line.chars()) {
            if character != '.' {
                antenna_locations
                    .entry(character)
                    .or_insert(Vec::new())
                    .push(vec![i as isize, j as isize])
            }

            characters.push(character);
        }

        map.push(characters);
    }

    let no_of_rows: isize = map.len() as isize;
    let no_of_columns: isize = map[0].len() as isize;

    println!();
    part_1(&no_of_rows, &no_of_columns, &antenna_locations);
    println!();
    part_2(&no_of_rows, &no_of_columns, &antenna_locations);

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "\nTotal Time Taken: {} seconds",
        total_time_taken.as_secs_f64()
    );
}
