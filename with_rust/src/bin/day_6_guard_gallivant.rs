use std::collections::HashSet;
use std::fs;
use std::time::{Duration, Instant};

use itertools::enumerate;

fn up(direction: &mut char, guard_position: &mut [isize; 2], obstacles: &HashSet<(isize, isize)>) {
    if !obstacles.contains(&(guard_position[0] - 1, guard_position[1])) {
        guard_position[0] -= 1;
    } else {
        *direction = 'r';
    }
}

fn right(
    direction: &mut char,
    guard_position: &mut [isize; 2],
    obstacles: &HashSet<(isize, isize)>,
) {
    if !obstacles.contains(&(guard_position[0], guard_position[1] + 1)) {
        guard_position[1] += 1;
    } else {
        *direction = 'd';
    }
}

fn down(
    direction: &mut char,
    guard_position: &mut [isize; 2],
    obstacles: &HashSet<(isize, isize)>,
) {
    if !obstacles.contains(&(guard_position[0] + 1, guard_position[1])) {
        guard_position[0] += 1;
    } else {
        *direction = 'l';
    }
}

fn left(
    direction: &mut char,
    guard_position: &mut [isize; 2],
    obstacles: &HashSet<(isize, isize)>,
) {
    if !obstacles.contains(&(guard_position[0], guard_position[1] - 1)) {
        guard_position[1] -= 1;
    } else {
        *direction = 'u';
    }
}

fn part_1(
    initial_guard_position: &[isize; 2],
    no_of_rows: &isize,
    no_of_columns: &isize,
    obstacles: &HashSet<(isize, isize)>,
) {
    let mut direction: char = 'u';
    let mut guard_position: [isize; 2] = initial_guard_position.clone();
    let mut guard_positions: HashSet<(isize, isize)> = HashSet::new();

    while (0 <= guard_position[0] && guard_position[0] < *no_of_rows)
        && (0 <= guard_position[1] && guard_position[1] < *no_of_columns)
    {
        guard_positions.insert((guard_position[0], guard_position[1]));

        if direction == 'u' {
            up(&mut direction, &mut guard_position, &obstacles);
        } else if direction == 'r' {
            right(&mut direction, &mut guard_position, &obstacles);
        } else if direction == 'd' {
            down(&mut direction, &mut guard_position, &obstacles);
        } else if direction == 'l' {
            left(&mut direction, &mut guard_position, &obstacles);
        }
    }

    println!("[+] Part One Solution");
    println!(">>> Sum: {}", guard_positions.len());
}

fn part_2(
    initial_guard_position: &[isize; 2],
    no_of_rows: &isize,
    no_of_columns: &isize,
    obstacles: &mut HashSet<(isize, isize)>,
) {
    let mut possible_positions_count: isize = 0;

    for i in 0..*no_of_rows {
        for j in 0..*no_of_columns {
            if obstacles.contains(&(i, j)) {
                continue;
            }

            let mut direction: char = 'u';
            let mut guard_position: [isize; 2] = initial_guard_position.clone();
            let mut guard_positions: HashSet<(isize, isize, char)> = HashSet::new();

            obstacles.insert((i, j));

            while (0 <= guard_position[0] && guard_position[0] < *no_of_rows)
                && (0 <= guard_position[1] && guard_position[1] < *no_of_columns)
            {
                if guard_positions.contains(&(guard_position[0], guard_position[1], direction)) {
                    possible_positions_count += 1;
                    break;
                }

                guard_positions.insert((guard_position[0], guard_position[1], direction));

                if direction == 'u' {
                    up(&mut direction, &mut guard_position, &obstacles);
                } else if direction == 'r' {
                    right(&mut direction, &mut guard_position, &obstacles);
                } else if direction == 'd' {
                    down(&mut direction, &mut guard_position, &obstacles);
                } else if direction == 'l' {
                    left(&mut direction, &mut guard_position, &obstacles);
                }
            }

            obstacles.remove(&(i, j));
        }
    }

    println!("[+] Part Two Solution");
    println!(">>> Possible Positions Count: {}", possible_positions_count);
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file = fs::read_to_string("../inputs/Day 6: Guard Gallivant/input.txt")
        .expect("Expecting a valid file");

    let mut obstacles: HashSet<(isize, isize)> = HashSet::new();
    let mut map: Vec<String> = Vec::new();
    let mut initial_guard_position: [isize; 2] = [0, 0];

    for (i, line) in enumerate(file.lines()) {
        for (j, character) in enumerate(line.chars()) {
            if character == '#' {
                obstacles.insert((i as isize, j as isize));
            }
            if character == '^' {
                initial_guard_position[0] = i as isize;
                initial_guard_position[1] = j as isize;
            }
        }

        map.push(line.to_string());
    }

    let no_of_rows: isize = map.len() as isize;
    let no_of_columns: isize = map[0].len() as isize;

    println!();
    part_1(
        &initial_guard_position,
        &no_of_rows,
        &no_of_columns,
        &obstacles,
    );
    println!();
    part_2(
        &initial_guard_position,
        &no_of_rows,
        &no_of_columns,
        &mut obstacles,
    );

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!("\nTotal Time Taken: {}", total_time_taken.as_secs_f64());
}
