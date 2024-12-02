use std::{fs, u16};

#[derive(PartialEq, Eq)]
enum LevelDirection {
    Increasing,
    Decreasing,
    Constant,
}

fn get_current_level_direction(prev_value: &i8, curr_value: &i8) -> LevelDirection {
    let direction: LevelDirection = if curr_value > prev_value {
        LevelDirection::Increasing
    } else if curr_value < prev_value {
        LevelDirection::Decreasing
    } else {
        LevelDirection::Constant
    };
    direction
}

fn is_distance_small(prev_value: &i8, curr_value: &i8) -> bool {
    let abs_distance = (curr_value - prev_value).abs();
    if 1 <= abs_distance && abs_distance <= 3 {
        true
    } else {
        false
    }
}

fn test_level_safety(level: Vec<i8>) -> bool {
    let first_of_level: &i8 = level.get(0).expect("Empty level");
    let second_of_level: &i8 = level.get(1).expect("Level with only one element");
    let level_direction = get_current_level_direction(first_of_level, second_of_level);
    for (prev, curr) in level.iter().zip(level.clone().iter().skip(1)) {
        let current_direction = get_current_level_direction(prev, curr);
        if current_direction != level_direction {
            return false;
        }
        let current_distance = is_distance_small(prev, curr);
        if !current_distance {
            return false;
        }
    }
    true
}

fn count_safe_levels(input_path: String) -> u16 {
    let contents = fs::read_to_string(input_path).expect("Trouble reading file");
    let mut safe_counter: u16 = 0;
    for line in contents.lines() {
        let level: Vec<i8> = line
            .split_whitespace()
            .map(|x| x.parse::<i8>().unwrap())
            .collect();
        let level_safe: bool = test_level_safety(level);
        if level_safe {
            safe_counter += 1
        }
    }
    safe_counter
}

pub fn complete_day_2(input_path: String) {
    let n_safe = count_safe_levels(input_path);
    println!("N safe levels: {}", n_safe)
}
