use regex::Regex;
use std::fs;
use std::time::{Duration, Instant};

fn part_1(content: &String) {
    let mut sum: u32 = 0;

    let regex_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for capture in regex_pattern.captures_iter(&content) {
        sum += capture.get(1).unwrap().as_str().parse::<u32>().unwrap()
            * capture.get(2).unwrap().as_str().parse::<u32>().unwrap()
    }

    println!("[+] Part One Solution");
    println!(">>> Sum of the results: {}", sum);
}

fn part_2(content: &String) {
    let mut sum: u32 = 0;
    let mut replaced_content = content.clone();

    let regex_pattern_1 = Regex::new(r"don't\(\).+?do\(\)").unwrap();

    for each in regex_pattern_1.find_iter(&content) {
        replaced_content = replaced_content.replace(each.as_str(), "");
    }

    let regex_pattern_2 = Regex::new(r"don't\(\).+?$").unwrap();

    replaced_content = replaced_content.replace(
        regex_pattern_2.find(&replaced_content).unwrap().as_str(),
        "",
    );

    let regex_pattern_3 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for capture in regex_pattern_3.captures_iter(&replaced_content) {
        sum += capture.get(1).unwrap().as_str().parse::<u32>().unwrap()
            * capture.get(2).unwrap().as_str().parse::<u32>().unwrap()
    }

    println!("[+] Part Two Solution");
    println!(">>> Sum of the results: {}", sum);
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let content = fs::read_to_string("../inputs/Day 3: Mull It Over/input.txt")
        .expect("Expecting a valid file.");

    println!();
    part_1(&content);
    println!();
    part_2(&content.replace("\n", ""));

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!();
    println!(
        "Total Time Taken: {} seconds",
        total_time_taken.as_secs_f64()
    )
}
