use itertools::enumerate;
use std::collections::HashSet;
use std::fs;
use std::time::{Duration, Instant};

fn get_next_positions(
    no_of_rows: &isize,
    no_of_columns: &isize,
    i: &isize,
    j: &isize,
) -> [Option<[isize; 2]>; 4] {
    let up: Option<[isize; 2]> = if i - 1 < 0 { None } else { Some([i - 1, *j]) };
    let down: Option<[isize; 2]> = if i + 1 < *no_of_rows {
        Some([i + 1, *j])
    } else {
        None
    };
    let left: Option<[isize; 2]> = if j - 1 < 0 { None } else { Some([*i, j - 1]) };
    let right: Option<[isize; 2]> = if j + 1 < *no_of_columns {
        Some([j + 1, *j])
    } else {
        None
    };

    [up, down, left, right]
}

fn get_valid_positions(
    map: &Vec<Vec<char>>,
    positions: &[Option<[isize; 2]>; 4],
    value: &isize,
) -> Vec<(isize, isize)> {
    let mut valid_positions: Vec<(isize, isize)> = Vec::new();

    for position in positions {
        if !position.is_none() {
            let [x, y] = position.unwrap();

            if map[x as usize][y as usize].to_digit(10).unwrap() == *value as u32 {
                valid_positions.push((x, y));
            }
        }
    }

    valid_positions
}

fn get_trailhead_rating_and_score(
    map: &Vec<Vec<char>>,
    no_of_rows: &isize,
    no_of_columns: &isize,
    i: &isize,
    j: &isize,
) -> (usize, usize) {
    let mut positions: Vec<(isize, isize)> = get_valid_positions(
        map,
        &get_next_positions(no_of_rows, no_of_columns, i, j),
        &1,
    );

    for value in 2..10 {
        let mut temp_positions: Vec<(isize, isize)> = Vec::new();

        for (x, y) in &positions {
            for valid_position in get_valid_positions(
                map,
                &get_next_positions(no_of_rows, no_of_columns, x, y),
                &value,
            ) {
                temp_positions.push(valid_position)
            }
        }

        positions = temp_positions
    }

    let rating: usize = positions.len();

    let score: HashSet<(isize, isize)> = positions.into_iter().collect();

    (rating, score.len())
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file: String = fs::read_to_string("../inputs/Day 10: Hoof It/sample-input.txt")
        .expect("Expecting a valid file.");

    let mut trailheads: Vec<(isize, isize)> = Vec::new();
    let mut map: Vec<Vec<char>> = Vec::new();

    for (i, line) in enumerate(file.lines()) {
        let mut temp: Vec<char> = Vec::new();

        for (j, character) in enumerate(line.chars()) {
            temp.push(character);

            if character == '0' {
                trailheads.push((i as isize, j as isize));
            }
        }

        map.push(temp);
    }

    let no_of_rows: isize = map.len() as isize;
    let no_of_columns: isize = map[0].len() as isize;

    let mut score: usize = 0;
    let mut rating: usize = 0;

    for (i, j) in trailheads {
        let (trailhead_rating, trailhead_score) =
            get_trailhead_rating_and_score(&map, &no_of_rows, &no_of_columns, &i, &j);

        score += trailhead_score;
        rating += trailhead_rating;
    }

    println!();
    println!("[+] Part One Solution");
    println!(">>> Sum of Scores of all Trailheads: {}", score);
    println!();
    println!("[+] Part Two Solution");
    println!(">>> Sum of Ratings of all Trailheads: {}", rating);

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!("\nTotal Time Taken: {}", total_time_taken.as_secs_f64());
}
