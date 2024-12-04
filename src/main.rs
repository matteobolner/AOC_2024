use std::fs;
mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
}
fn day1() {
    let day_1_input = String::from("input/day1/input.txt");
    days::day1::complete_day_1(day_1_input)
}
fn day2() {
    let day_2_input = String::from("input/day2/input.txt");
    days::day2::complete_day_2(day_2_input)
}

fn day3() {
    let day_3_input = String::from("input/day3/input.txt");
    days::day3::complete_day_3(day_3_input)
}

//fn get_neighboring_positions(matrix: Vec<Vec<char>>, x: i32, y: i32) {}

fn find_char_in_neighbours(
    matrix: Vec<Vec<char>>,
    x: i32,
    y: i32,
    seeked_char: char,
) -> Vec<(i32, i32)> {
    let matrix_width = matrix[0].len();
    let matrix_height = matrix.len();
    let mut found_char_coords: Vec<(i32, i32)> = Vec::new();
    for x_iter in x - 1..x + 2 {
        for y_iter in y - 1..y + 2 {
            if x_iter != x && y_iter != y {
                continue;
            }

            if x_iter >= 0
                && y_iter >= 0
                && x_iter < matrix_width as i32
                && y_iter < matrix_height as i32
            {
                //println!("{} {}", x_iter, y_iter);

                if matrix[x_iter as usize][y_iter as usize] == seeked_char {
                    println!("{} {} {} {}", x, y, x_iter, y_iter);
                    found_char_coords.push((x_iter, y_iter));
                }
            }
        }
    }
    found_char_coords
}

fn main() {
    let input_path = String::from("input/day4/input.txt");
    let contents = fs::read_to_string(input_path).expect("Trouble reading file");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for i in contents.lines() {
        matrix.push(i.chars().collect())
    }
    let matrix_width = matrix[0].len();
    let matrix_height = matrix.len();
    let found = find_char_in_neighbours(matrix, 1, 0, 'X');
    println!("{:#?}", found);
}
