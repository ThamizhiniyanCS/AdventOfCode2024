use regex::Regex;
use std::fs;
use std::time::{Duration, Instant};

fn cramers_rule(
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
) -> (isize, isize) {
    let (a_1, a_2) = button_a;
    let (b_1, b_2) = button_b;
    let (c_1, c_2) = prize;

    let determinant: isize = (a_1 * b_2) - (b_1 * a_2);
    let determinant_x: isize = (c_1 * b_2) - (b_1 * c_2);
    let determinant_y: isize = (a_1 * c_2) - (c_1 * a_2);

    if determinant_x % determinant == 0 && determinant_y % determinant == 0 {
        let x: isize = determinant_x / determinant;
        let y: isize = determinant_y / determinant;

        return (x, y);
    }

    (0, 0)
}

fn calculate_tokens(presses: (isize, isize)) -> isize {
    let (x, y) = presses;

    (x * 3) + (y * 1)
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let file: String = fs::read_to_string("../inputs/Day 13: Claw Contraption/input.txt")
        .expect("Expecting a valid file.");

    let mut input_list: Vec<String> = Vec::new();

    for line in file.lines() {
        let trim_line = line.trim();

        if !trim_line.is_empty() {
            input_list.push(trim_line.to_string());
        }
    }

    let mut part_2_sum: isize = 0;
    let mut part_1_sum: isize = 0;

    let regex_button_a_pattern = Regex::new(r"Button\sA:\sX\+(\d{1,2}),\sY\+(\d{1,2})").unwrap();
    let regex_button_b_pattern = Regex::new(r"Button\sB:\sX\+(\d{1,2}),\sY\+(\d{1,2})").unwrap();
    let regex_prize_pattern = Regex::new(r"Prize:\sX=(\d+?),\sY=(\d+?)\b").unwrap();

    for batch in input_list.chunks(3) {
        let input_button_a = &batch[0];
        let input_button_b = &batch[1];
        let input_prize = &batch[2];

        let captures_button_a = regex_button_a_pattern.captures(&input_button_a).unwrap();
        let captures_button_b = regex_button_b_pattern.captures(&input_button_b).unwrap();
        let captures_prize = regex_prize_pattern.captures(&input_prize).unwrap();

        let button_a: (isize, isize) = (
            captures_button_a
                .get(1)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
            captures_button_a
                .get(2)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
        );
        let button_b: (isize, isize) = (
            captures_button_b
                .get(1)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
            captures_button_b
                .get(2)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
        );
        let prize: (isize, isize) = (
            captures_prize
                .get(1)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
            captures_prize
                .get(2)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
        );

        part_1_sum += calculate_tokens(cramers_rule(button_a, button_b, prize));
        part_2_sum += calculate_tokens(cramers_rule(
            button_a,
            button_b,
            (prize.0 + 10000000000000, prize.1 + 10000000000000),
        ));
    }

    println!();
    println!("[+] Part One Solution");
    println!(
        ">>> The fewest tokens required to win all possible prizes: {}",
        part_1_sum
    );
    println!();
    println!("[+] Part Two Solution");
    println!(
        ">>> The fewest tokens required to win all possible prizes: {}",
        part_2_sum
    );

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "\nTotal Time Taken: {} seconds.",
        total_time_taken.as_secs_f64()
    );
}
