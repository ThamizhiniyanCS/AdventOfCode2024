use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};
use std::{collections, isize};

fn get_positions(
    no_of_rows: &isize,
    no_of_columns: &isize,
    i: &isize,
    j: &isize,
) -> HashMap<String, Option<[isize; 2]>> {
    let i_minus_1: Option<isize> = if i - 1 >= 0 { Some(i - 1) } else { None };
    let i_plus_1: Option<isize> = if i + 1 < *no_of_rows {
        Some(i + 1)
    } else {
        None
    };
    let j_minus_1: Option<isize> = if j - 1 >= 0 { Some(j - 1) } else { None };
    let j_plus_1: Option<isize> = if j + 1 < *no_of_columns {
        Some(j + 1)
    } else {
        None
    };

    let mut positions: HashMap<String, Option<[isize; 2]>> = collections::HashMap::new();

    positions.insert(
        "left".to_string(),
        if j_minus_1 != None {
            Some([*i, j_minus_1.unwrap()])
        } else {
            None
        },
    );
    positions.insert(
        "right".to_string(),
        if j_plus_1 != None {
            Some([*i, j_plus_1.unwrap()])
        } else {
            None
        },
    );
    positions.insert(
        "up".to_string(),
        if i_minus_1 != None {
            Some([i_minus_1.unwrap(), *j])
        } else {
            None
        },
    );
    positions.insert(
        "down".to_string(),
        if i_plus_1 != None {
            Some([i_plus_1.unwrap(), *j])
        } else {
            None
        },
    );
    positions.insert(
        "top_left".to_string(),
        if i_minus_1 != None && j_minus_1 != None {
            Some([i_minus_1.unwrap(), j_minus_1.unwrap()])
        } else {
            None
        },
    );
    positions.insert(
        "top_right".to_string(),
        if i_minus_1 != None && j_plus_1 != None {
            Some([i_minus_1.unwrap(), j_plus_1.unwrap()])
        } else {
            None
        },
    );
    positions.insert(
        "bottom_left".to_string(),
        if i_plus_1 != None && j_minus_1 != None {
            Some([i_plus_1.unwrap(), j_minus_1.unwrap()])
        } else {
            None
        },
    );
    positions.insert(
        "bottom_right".to_string(),
        if i_plus_1 != None && j_plus_1 != None {
            Some([i_plus_1.unwrap(), j_plus_1.unwrap()])
        } else {
            None
        },
    );

    return positions;
}

fn find_character_positions(
    no_of_rows: &isize,
    no_of_columns: &isize,
    content: &Vec<Vec<char>>,
    character: char,
) -> Vec<[isize; 2]> {
    let mut positions: Vec<[isize; 2]> = Vec::new();

    for i in 0..*no_of_rows {
        for j in 0..*no_of_columns {
            if content[i as usize][j as usize] == character {
                positions.push([i, j]);
            }
        }
    }

    return positions;
}

fn find_m_positions(
    no_of_rows: &isize,
    no_of_columns: &isize,
    content: &Vec<Vec<char>>,
    i: &isize,
    j: &isize,
) -> HashMap<String, Option<[isize; 2]>> {
    let mut positions: HashMap<String, Option<[isize; 2]>> = collections::HashMap::new();

    for (key, value) in get_positions(no_of_rows, no_of_columns, i, j) {
        if value != None && content[value.unwrap()[0] as usize][value.unwrap()[1] as usize] == 'M' {
            positions.insert(key, value);
        }
    }

    return positions;
}

fn check_character(
    no_of_rows: &isize,
    no_of_columns: &isize,
    content: &Vec<Vec<char>>,
    direction: &String,
    i: &isize,
    j: &isize,
    character: char,
) -> Option<[isize; 2]> {
    let positions: HashMap<String, Option<[isize; 2]>> =
        get_positions(no_of_rows, no_of_columns, i, j);

    if positions[direction] != None
        && content[positions[direction].unwrap()[0] as usize]
            [positions[direction].unwrap()[1] as usize]
            == character
    {
        positions[direction]
    } else {
        None
    }
}

fn check_x_mas(
    no_of_rows: &isize,
    no_of_columns: &isize,
    content: &Vec<Vec<char>>,
    i: &isize,
    j: &isize,
) -> bool {
    let positions: HashMap<String, Option<[isize; 2]>> =
        get_positions(no_of_rows, no_of_columns, i, j);

    let position_top_left: &Option<[isize; 2]> = positions.get("top_left").unwrap();
    let position_top_right: &Option<[isize; 2]> = positions.get("top_right").unwrap();
    let position_bottom_left: &Option<[isize; 2]> = positions.get("bottom_left").unwrap();
    let position_bottom_right: &Option<[isize; 2]> = positions.get("bottom_right").unwrap();

    if *position_top_left != None
        && *position_top_right != None
        && *position_bottom_left != None
        && *position_bottom_right != None
    {
        let top_left: char =
            content[position_top_left.unwrap()[0] as usize][position_top_left.unwrap()[1] as usize];
        let top_right: char = content[position_top_right.unwrap()[0] as usize]
            [position_top_right.unwrap()[1] as usize];
        let bottom_left: char = content[position_bottom_left.unwrap()[0] as usize]
            [position_bottom_left.unwrap()[1] as usize];
        let bottom_right: char = content[position_bottom_right.unwrap()[0] as usize]
            [position_bottom_right.unwrap()[1] as usize];

        if (top_left == 'M' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'S')
            || (top_left == 'M' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'S')
            || (top_left == 'S' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'M')
            || (top_left == 'S' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'M')
        {
            return true;
        }
    }

    false
}

fn part_1(content: &Vec<Vec<char>>, no_of_rows: &isize, no_of_columns: &isize) {
    let mut total_times: isize = 0;

    for x in find_character_positions(no_of_rows, no_of_columns, content, 'X') {
        for (key, value) in find_m_positions(no_of_rows, no_of_columns, content, &x[0], &x[1]) {
            let result_a: Option<[isize; 2]> = check_character(
                no_of_rows,
                no_of_columns,
                content,
                &key,
                &value.unwrap()[0],
                &value.unwrap()[1],
                'A',
            );

            if result_a != None {
                let result_s: Option<[isize; 2]> = check_character(
                    no_of_rows,
                    no_of_columns,
                    content,
                    &key,
                    &result_a.unwrap()[0],
                    &result_a.unwrap()[1],
                    'S',
                );

                if result_s != None {
                    total_times += 1;
                }
            }
        }
    }

    println!("[+] Part One Solution");
    println!(">>> Total Times XMAS Appeared: {}", total_times);
}

fn part_2(content: &Vec<Vec<char>>, no_of_rows: &isize, no_of_columns: &isize) {
    let mut total_times: isize = 0;

    for a in find_character_positions(no_of_rows, no_of_columns, content, 'A') {
        if check_x_mas(no_of_rows, no_of_columns, content, &a[0], &a[1]) {
            total_times += 1
        }
    }

    println!("[+] Part Two Solution");
    println!(">>> Total Times X-MAS Appeared: {}", total_times);
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file: Vec<Vec<char>> = fs::read_to_string("../inputs/Day 4: Ceres Search/input.txt")
        .expect("Expecting a valid file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let no_of_rows: isize = file.len() as isize;
    let no_of_columns: isize = file[0].len() as isize;

    println!();
    part_1(&file, &no_of_rows, &no_of_columns);
    println!();
    part_2(&file, &no_of_rows, &no_of_columns);

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "\nTotal Time Taken: {} seconds.",
        total_time_taken.as_secs_f64()
    )
}
