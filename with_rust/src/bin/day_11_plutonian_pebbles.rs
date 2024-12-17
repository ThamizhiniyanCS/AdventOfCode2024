use cached::proc_macro::cached;
use std::fs;
use std::time::{Duration, Instant};

// My Initial Solution
// fn blink(stones: &Vec<String>) -> Vec<String> {
//     let mut temp: Vec<String> = Vec::new();

//     for stone in stones {
//         if stone == "0" {
//             temp.push("1".to_string());
//         } else if stone.len() % 2 == 0 {
//             let divide_len = stone.len() / 2;

//             let first_part: String = stone[..divide_len].parse::<isize>().unwrap().to_string();
//             let second_part: String = stone[divide_len..].parse::<isize>().unwrap().to_string();

//             temp.push(first_part.clone());
//             temp.push(second_part.clone());
//         } else {
//             match stone.parse::<isize>() {
//                 Ok(number) => {
//                     let result: String = (number * 2024).to_string();
//                     temp.push(result.clone());
//                 }
//                 Err(_) => println!("Failed to parse '{}' as isize", stone),
//             }
//         }
//     }

//     temp
// }

// https://www.youtube.com/watch?v=JxIAqiUJraE
#[cached]
fn count_stones(stone: String, blinks: isize) -> isize {
    if blinks == 0 {
        return 1;
    }

    if stone == "0" {
        return count_stones("1".to_string(), blinks - 1);
    }

    let len_stone = stone.len();
    if len_stone % 2 == 0 {
        let divide_len = stone.len() / 2;
        let first_part: String = stone[..divide_len].parse::<isize>().unwrap().to_string();
        let second_part: String = stone[divide_len..].parse::<isize>().unwrap().to_string();

        return count_stones(first_part, blinks - 1) + count_stones(second_part, blinks - 1);
    }

    match stone.parse::<isize>() {
        Ok(number) => count_stones((number * 2024).to_string(), blinks - 1),
        Err(_) => panic!("Failed to parse '{}' as isize", stone),
    }
}

fn part_1(initial_agreement: &String) {
    let stones: Vec<String> = initial_agreement
        .clone()
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut sum = 0;

    for stone in stones {
        sum += count_stones(stone, 25);
    }

    println!("[+] Part One Solution");
    println!(">>> Number of tones after 25 blinks: {}", sum);
}

fn part_2(initial_agreement: &String) {
    let stones: Vec<String> = initial_agreement
        .clone()
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut sum = 0;

    for stone in stones {
        sum += count_stones(stone, 75);
    }

    println!("[+] Part Two Solution");
    println!(">>> Number of tones after 75 blinks: {}", sum);
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let initial_agreement: String =
        fs::read_to_string("../inputs/Day 11: Plutonian Pebbles/input.txt")
            .expect("Expecting a valid file.")
            .trim()
            .to_string();

    part_1(&initial_agreement);
    println!();
    part_2(&initial_agreement);

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!("\nTotal Time Taken: {}", total_time_taken.as_secs_f64());
}
