use std::fs;

fn parse_input_and_return_vecs(input_path: String) -> (Vec<i32>, Vec<i32>) {
    let mut first_vec: Vec<i32> = Vec::new();
    let mut second_vec: Vec<i32> = Vec::new();
    let contents = fs::read_to_string(input_path).expect("Trouble reading file");
    for line in contents.lines() {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        first_vec.push(split_line[0].parse::<i32>().unwrap());
        second_vec.push(split_line[1].parse::<i32>().unwrap())
    }
    (first_vec, second_vec)
}

fn sorted_vecs_diff(mut first_vec: Vec<i32>, mut second_vec: Vec<i32>) -> i32 {
    first_vec.sort();
    second_vec.sort();
    let mut total_diff: i32 = 0;
    for (i, j) in first_vec.iter().zip(second_vec.iter()) {
        let size_diff = (i - j).abs();
        total_diff += size_diff
    }
    total_diff
}

fn sorted_vecs_similarity_score(first_vec: Vec<i32>, second_vec: Vec<i32>) -> i32 {
    let mut similarity_score: i32 = 0;
    for i in first_vec.iter() {
        let appearance_counts = second_vec.iter().filter(|n| *n == i).count();
        similarity_score += i * appearance_counts as i32
    }
    similarity_score
}

pub fn complete_day_1(input_path: String) {
    let (first_vec, second_vec) = parse_input_and_return_vecs(input_path);
    let sorted_result = sorted_vecs_diff(first_vec.clone(), second_vec.clone());
    println!("Total sum difference: {}", sorted_result);
    let similarity_score = sorted_vecs_similarity_score(first_vec, second_vec);
    println!("Similarity score: {}", similarity_score);
}
