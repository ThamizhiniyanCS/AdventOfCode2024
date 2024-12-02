use std::fs;
use std::time::{Duration, Instant};

fn _is_safe_report() {}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file = fs::read_to_string("../inputs/Day 2: Red-Nosed Reports/sample-input.txt")
        .expect("Expecting a valid file.");

    let unsafe_reports: Vec<Vec<u32>> = file
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<u32>().expect("Expecting a valid integer."))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "Total Time Taken: {} seconds",
        total_time_taken.as_secs_f64()
    )
}
