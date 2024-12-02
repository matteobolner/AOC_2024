use std::{fs, u16};

#[derive(PartialEq, Eq)]
enum ReportDirection {
    Increasing,
    Decreasing,
    Constant,
}

fn get_current_report_direction(prev_value: &i8, curr_value: &i8) -> ReportDirection {
    let direction: ReportDirection = if curr_value > prev_value {
        ReportDirection::Increasing
    } else if curr_value < prev_value {
        ReportDirection::Decreasing
    } else {
        ReportDirection::Constant
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

fn test_report_safety(report: Vec<i8>, dampened: bool) -> bool {
    let first_level: &i8 = report.get(0).expect("Empty report");
    let second_level: &i8 = report.get(1).expect("report with only one element");
    let report_direction = get_current_report_direction(first_level, second_level);
    for (prev, curr) in report.iter().zip(report.clone().iter().skip(1)) {
        let current_direction = get_current_report_direction(prev, curr);
        let current_distance = is_distance_small(prev, curr);

        if current_direction != report_direction || !current_distance {
            if !dampened {
                for (index, _value) in report.iter().enumerate() {
                    let mut dampened_report = report.clone();
                    dampened_report.remove(index);
                    if test_report_safety(dampened_report, true) {
                        return true;
                    }
                }
            }
            {
                return false;
            }
        }
    }
    true
}

fn count_safe_reports(input_path: String, dampener: bool) -> u16 {
    let contents = fs::read_to_string(input_path).expect("Trouble reading file");
    let mut safe_counter: u16 = 0;
    for line in contents.lines() {
        let report: Vec<i8> = line
            .split_whitespace()
            .map(|x| x.parse::<i8>().unwrap())
            .collect();
        let report_safe: bool = match dampener {
            true => test_report_safety(report, false),
            false => test_report_safety(report, true),
        };
        if report_safe {
            safe_counter += 1
        }
    }
    safe_counter
}

pub fn complete_day_2(input_path: String) {
    let n_safe_normal = count_safe_reports(input_path.clone(), false);
    let n_safe_dampened = count_safe_reports(input_path.clone(), true);
    println!("N safe reports without dampener: {}", n_safe_normal);
    println!("N safe reports with dampener: {}", n_safe_dampened)
}
