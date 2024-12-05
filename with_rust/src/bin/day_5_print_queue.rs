use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};

fn sum_of_middle(valid_updates: &Vec<Vec<usize>>) -> usize {
    let mut sum: usize = 0;

    for update in valid_updates {
        sum += update[(update.len() - 1) / 2];
    }

    sum
}

fn part_1(dictionary: &HashMap<usize, Vec<usize>>, updates: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut valid_updates: Vec<Vec<usize>> = Vec::new();
    let mut invalid_updates: Vec<Vec<usize>> = Vec::new();

    for i in 0..updates.len() {
        let mut results: Vec<bool> = Vec::new();

        for j in 0..(updates[i].len() - 1) {
            let current_value: usize = updates[i][j];

            if let Some(next_value_dictionary) = dictionary.get(&updates[i][j + 1]) {
                if next_value_dictionary.contains(&current_value) {
                    results.push(false)
                } else {
                    results.push(true)
                }
            }
        }

        if results.contains(&false) {
            invalid_updates.push(updates[i].clone());
        } else {
            valid_updates.push(updates[i].clone());
        }
    }

    println!("[+] Part One Solution");
    println!(
        ">>> Sum of middle values of valid updates: {}",
        sum_of_middle(&valid_updates)
    );

    invalid_updates
}

fn part_2(dictionary: &HashMap<usize, Vec<usize>>, invalid_updates: &Vec<Vec<usize>>) {
    let mut valid_updates: Vec<Vec<usize>> = invalid_updates.clone();

    for i in 0..invalid_updates.len() {
        for _ in 0..invalid_updates[i].len() {
            for j in 0..(invalid_updates[i].len() - 1) {
                let current_value: usize = valid_updates[i][j];

                if let Some(next_value_dictionary) = dictionary.get(&valid_updates[i][j + 1]) {
                    if next_value_dictionary.contains(&current_value) {
                        valid_updates[i][j] = valid_updates[i][j + 1];
                        valid_updates[i][j + 1] = current_value;
                    }
                }
            }
        }
    }

    println!("[+] Part Two Solution");
    println!(
        ">>> Sum of middle values of valid updates: {}",
        sum_of_middle(&valid_updates)
    );
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file: Vec<String> = fs::read_to_string("../inputs/Day 5: Print Queue/input.txt")
        .expect("Expecting a valid file")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut dictionary: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();

    for line in &file[0..1176] {
        let values: Vec<usize> = line
            .split("|")
            .map(|value| value.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        dictionary
            .entry(values[0])
            .or_insert(Vec::new())
            .push(values[1]);
    }

    for line in &file[1177..1367] {
        let values: Vec<usize> = line
            .split(",")
            .map(|value| value.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        updates.push(values);
    }

    println!();
    let invalid_updates: Vec<Vec<usize>> = part_1(&dictionary, &updates);
    println!();
    part_2(&dictionary, &invalid_updates);

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "\nTotal Time Taken: {} seconds.",
        total_time_taken.as_secs_f64()
    );
}
