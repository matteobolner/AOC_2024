use regex::Regex;
use std::{fs, u16};

fn find_regex(input_string: String) -> Vec<String> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let matches_vec: Vec<String> = re
        .find_iter(&input_string)
        .map(|x| String::from(x.as_str()))
        .collect();
    matches_vec
}

fn extract_and_multiply(input_string: String) -> u64 {
    let trimmed_string = input_string.replace("mul(", "").replace(")", "");
    let numbers: Vec<u64> = trimmed_string
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let multiplication: u64 = numbers[0] * numbers[1];
    multiplication
}

pub fn complete_day_3(input: String) {
    let input_path = "input/day3/input.txt";
    let contents = fs::read_to_string(input_path).expect("Trouble reading file");
    let matched = find_regex(contents);
    let mut total_sum = 0;
    for i in matched.iter() {
        total_sum += extract_and_multiply(String::from(i))
    }
    println!("Total sum: {}", total_sum)
}
