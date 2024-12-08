use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::{Duration, Instant};

use itertools::enumerate;

fn is_valid(elements: Vec<isize>, min: isize, max: isize) -> bool {
    for each in elements {
        if !min <= each && !each < max {
            return false;
        }
    }
    true
}

fn part_1(
    no_of_rows: &usize,
    no_of_columns: &usize,
    antenna_locations: &HashMap<char, Vec<Vec<isize>>>,
) {
    let antinode_locations: HashSet<(isize; 2)> = HashSet::new();
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file = fs::read_to_string("../inputs/Day 8: Resonant Collinearity/sample-input.txt")
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

    let no_of_rows: usize = map.len();
    let no_of_columns: usize = map[0].len();

    println!();
    part_1(&no_of_rows, &no_of_columns, &antenna_locations);
    println!();

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "\nTotal Time Taken: {} seconds",
        total_time_taken.as_secs_f64()
    );
}
