use std::fs;
use std::time::{Duration, Instant};

fn is_safe_report(report: &Vec<u32>) -> bool {
    let mut order = "";

    for i in 0..(report.len() - 1) {
        let a = report[i];
        let b = report[i + 1];

        let difference = a.abs_diff(b);

        if 1 <= difference && difference <= 3 {
            if order == "" {
                if a > b {
                    order = ">"
                } else {
                    order = "<"
                }
            }

            if order == ">" && !(a > b) {
                return false;
            }

            if order == "<" && !(a < b) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn part_1(file: String) -> (u32, Vec<Vec<u32>>) {
    let mut total_safe_reports: u32 = 0;
    let mut unsafe_reports: Vec<Vec<u32>> = Vec::new();

    for report in file.lines().map(|line| {
        line.split_whitespace()
            .map(|value| value.parse::<u32>().expect("Expecting a valid integer."))
            .collect::<Vec<u32>>()
    }) {
        if is_safe_report(&report) {
            total_safe_reports += 1;
        } else {
            unsafe_reports.push(report);
        }
    }

    println!("[+] Part One Solution");
    println!(">>> Total Safe Reports: {}", total_safe_reports);

    return (total_safe_reports, unsafe_reports);
}

fn part_2(total_safe_reports: &mut u32, unsafe_reports: &Vec<Vec<u32>>) {
    for report in unsafe_reports {
        for i in 0..report.len() {
            let mut report_copy = report.to_vec();
            report_copy.remove(i);

            if is_safe_report(&report_copy) {
                *total_safe_reports += 1;
                break;
            }
        }
    }

    println!("[+] Part Two Solution");
    println!(
        ">>> Total Safe Reports after implementing Problem Dampener: {}",
        total_safe_reports
    );
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file = fs::read_to_string("../inputs/Day 2: Red-Nosed Reports/input.txt")
        .expect("Expecting a valid file.");

    println!();
    let (mut total_safe_reports, unsafe_reports) = part_1(file);
    println!();
    part_2(&mut total_safe_reports, &unsafe_reports);

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "Total Time Taken: {} seconds",
        total_time_taken.as_secs_f64()
    )
}
