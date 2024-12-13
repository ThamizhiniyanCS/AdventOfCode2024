use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::time::{Duration, Instant};

use itertools::{enumerate, Itertools};

fn part_1(file_system: &Vec<Option<isize>>) {
    let mut file_system_copy: Vec<Option<isize>> = file_system.clone();
    let mut file_system_checksum: isize = 0;

    while file_system_copy.iter().any(|file| file.is_none()) {
        let index: usize = file_system_copy.iter().position(|p| p.is_none()).unwrap();

        if index < file_system_copy.len() - 1 {
            file_system_copy[index] = file_system_copy.pop().unwrap();
        } else {
            file_system_copy.pop();
        }
    }

    for (i, file_id) in enumerate(file_system_copy) {
        file_system_checksum += i as isize * file_id.unwrap();
    }

    println!("[+] Part One Solution");
    println!(">>> File System Checksum: {}", file_system_checksum);
}

fn part_2(file_system: &Vec<Option<isize>>, character_occurances: &HashMap<isize, isize>) {
    let mut file_system_copy: Vec<Option<isize>> = file_system.clone();
    let len_file_system_copy: usize = file_system_copy.len();
    let mut file_system_checksum: isize = 0;

    for id in character_occurances.keys().sorted().rev() {
        let id_start_index = file_system_copy
            .iter()
            .position(|p: &Option<isize>| p.as_ref() == Some(id))
            .unwrap();

        let mut file_indexes: Vec<usize> = Vec::new();

        for i in id_start_index..(id_start_index + (character_occurances[id] as usize)) {
            file_indexes.push(i);
        }

        let empty_space_start_index: usize = file_system_copy
            .iter()
            .position(|p: &Option<isize>| p.is_none())
            .unwrap();

        let mut empty_space_indexes: Vec<usize> = Vec::new();

        for i in empty_space_start_index..len_file_system_copy {
            if file_system_copy[i].is_none() {
                empty_space_indexes.push(i)
            } else {
                if empty_space_indexes.len() >= character_occurances[id] as usize {
                    break;
                } else {
                    empty_space_indexes = Vec::new();
                }
            }
        }

        for (file, space) in zip(file_indexes, empty_space_indexes) {
            if space < file {
                file_system_copy[space] = file_system_copy[file];
                file_system_copy[file] = None
            }
        }
    }

    for (i, file_id) in enumerate(file_system_copy) {
        if !file_id.is_none() {
            file_system_checksum += i as isize * file_id.unwrap();
        }
    }

    println!("[+] Part Two Solution");
    println!(">>> File System Checksum: {}", file_system_checksum);
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file = fs::read_to_string("../inputs/Day 9: Disk Fragmenter/input.txt")
        .expect("Expecting a valid file");

    let input: Vec<isize> = file
        .trim()
        .chars()
        .map(|char: char| char.to_digit(10).unwrap() as isize)
        .collect::<Vec<isize>>();

    let mut pairs: Vec<[isize; 2]> = Vec::new();

    for i in (0..input.len()).step_by(2) {
        if i + 1 < input.len() {
            pairs.push([input[i], input[i + 1]]);
        } else {
            pairs.push([input[i], -1]);
        }
    }

    let mut file_system: Vec<Option<isize>> = Vec::new();
    let mut character_occurances: HashMap<isize, isize> = HashMap::new();

    for (i, pair) in enumerate(&pairs) {
        if i < &pairs.len() - 1 {
            for _ in 0..pair[0] {
                file_system.push(Some(i as isize));
                *character_occurances.entry(i as isize).or_insert(0) += 1
            }

            for _ in 0..pair[1] {
                file_system.push(None)
            }
        } else {
            for _ in 0..pair[0] {
                file_system.push(Some(i as isize));
                *character_occurances.entry(i as isize).or_insert(0) += 1
            }
        }
    }

    println!();
    part_1(&file_system);
    println!();
    part_2(&file_system, &character_occurances);

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "\nTotal Time Taken: {} seconds.",
        total_time_taken.as_secs_f64()
    )
}
