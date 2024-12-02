use std::collections;
use std::fs;
use std::time::{Duration, Instant};

fn part_1(list_1: &[i32], list_2: &[i32]) {
    let mut sorted_list_1 = list_1.to_vec();
    let mut sorted_list_2 = list_2.to_vec();

    sorted_list_1.sort_unstable();
    sorted_list_2.sort_unstable();

    let total_distance: i32 = sorted_list_1
        .iter()
        .zip(sorted_list_2)
        .map(|(value_1, value_2)| (value_1 - value_2).abs())
        .sum();

    println!("[+] Part One Solution");
    println!(">>> Total Distance: {}", total_distance);
}

fn part_2(list_1: &[i32], list_2: &[i32]) {
    let mut hashmap: collections::HashMap<i32, i32> = collections::HashMap::new();

    for &value in list_2 {
        hashmap
            .entry(value)
            .and_modify(|counter| *counter += 1)
            .or_insert(0);
    }

    let similarity_score: i32 = list_1
        .iter()
        .map(|value| value * hashmap.get(&value).copied().unwrap_or(0))
        .sum();

    println!("[+] Part Two Solution");
    println!(">>> Similarity Score: {}", similarity_score);
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file = fs::read_to_string("../inputs/Day 1: Historian Hysteria/input.txt")
        .expect("Expecting a valid file.");

    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();

    for line in file.lines() {
        let mut values = line
            .split_whitespace()
            .map(|value| value.parse::<i32>().expect("Expecting a valid integer."))
            .take(2);

        list_1.push(values.next().unwrap());
        list_2.push(values.next().unwrap());
    }

    println!();
    part_1(&list_1, &list_2);
    println!();
    part_2(&list_1, &list_2);

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "Total Time Taken: {} seconds",
        total_time_taken.as_secs_f64()
    );
}
