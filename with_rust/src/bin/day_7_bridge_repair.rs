use itertools::{Itertools, MultiProduct};
use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};

// Reference: https://stackoverflow.com/a/68231315
fn product_repeat<I>(it: I, repeat: usize) -> MultiProduct<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    std::iter::repeat(it).take(repeat).multi_cartesian_product()
}

fn isvalid_combo(key: &isize, value: &Vec<String>, combo: &Vec<&&str>) -> bool {
    let mut res: isize = 0;
    let mut operator: &str = "+";
    let mut combo = combo.clone();

    combo.push(&"");

    for each in value.iter().zip(combo) {
        if operator == "+" {
            res += each.0.parse::<isize>().unwrap();
        }
        if operator == "*" {
            res *= each.0.parse::<isize>().unwrap();
        }
        if operator == "||" {
            res = (res.to_string() + each.0).parse::<isize>().unwrap();
        }
        operator = each.1;
    }

    if res == *key {
        return true;
    } else {
        return false;
    }
}

fn part_1(equations: HashMap<isize, Vec<String>>) -> (HashMap<isize, Vec<String>>, isize) {
    let mut sum: isize = 0;
    let operators: [&str; 2] = ["+", "*"];
    let mut invalid_equations: HashMap<isize, Vec<String>> = HashMap::new();

    for (key, value) in equations {
        let mut isvalid: bool = false;

        for combo in product_repeat(operators.iter(), value.len() - 1) {
            if isvalid_combo(&key, &value, &combo) {
                sum += key;
                isvalid = true;
                break;
            }
        }

        if !isvalid {
            invalid_equations.entry(key).or_insert(value);
        }
    }

    println!("[+] Part One Solution");
    println!(">>> Sum: {}", sum);

    return (invalid_equations, sum);
}

fn part_2(invalid_equations: HashMap<isize, Vec<String>>, sum: isize) {
    let operators: [&str; 3] = ["+", "*", "||"];
    let mut sum = sum;

    for (key, value) in invalid_equations {
        for combo in product_repeat(operators.iter(), value.len() - 1) {
            if isvalid_combo(&key, &value, &combo) {
                sum += key;
                break;
            }
        }
    }

    println!("[+] Part Two Solution");
    println!(">>> Sum: {}", sum);
}

fn main() {
    let total_time_taken_start: Instant = Instant::now();

    let mut equations: HashMap<isize, Vec<String>> = HashMap::new();

    let file = fs::read_to_string("../inputs/Day 7: Bridge Repair/input.txt")
        .expect("Expecting a valid file.")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    for line in file {
        let temp: Vec<&str> = line.split(":").collect::<Vec<&str>>();
        equations
            .entry(temp[0].parse::<isize>().unwrap())
            .or_insert(
                temp[1]
                    .trim()
                    .split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            );
    }

    println!();
    let (invalid_equations, sum) = part_1(equations);
    println!();
    part_2(invalid_equations, sum);

    let total_time_taken: Duration = Instant::elapsed(&total_time_taken_start);

    println!(
        "\nTotal Time Taken: {} seconds.",
        total_time_taken.as_secs_f64()
    );
}
