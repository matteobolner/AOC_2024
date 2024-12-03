use regex::Regex;
use std::fs;

fn find_regex(input_string: String, regex_string: String) -> Vec<String> {
    let re = Regex::new(&regex_string).unwrap();
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

pub fn complete_day_3(input_path: String) {
    let mut contents = fs::read_to_string(input_path).expect("Trouble reading file");
    contents = contents.replace("\n", "");

    let matched = find_regex(contents.clone(), String::from(r"mul\(\d+,\d+\)"));
    let mut total_sum = 0;
    for i in matched.iter() {
        total_sum += extract_and_multiply(String::from(i))
    }
    println!("Total sum: {}", total_sum);
    let matched_to_remove = find_regex(contents.clone(), String::from(r"don't\(\).*?do\(\)"));
    for i in matched_to_remove.iter() {
        contents = contents.replace(i, "");
    }
    let last_dont = find_regex(contents.clone(), String::from(r"don't\(\).*"));
    for i in last_dont.iter() {
        contents = contents.replace(i, "");
    }
    let matched_trimmed = find_regex(contents, String::from(r"mul\(\d+,\d+\)"));
    let mut total_sum_trimmed = 0;
    for i in matched_trimmed.iter() {
        total_sum_trimmed += extract_and_multiply(String::from(i))
    }
    println!("Total sum trimmed: {}", total_sum_trimmed);
}
