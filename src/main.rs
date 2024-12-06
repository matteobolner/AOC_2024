use std::{cmp, fs};
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

enum MatrixDiagonalOrientation {
    Left,
    Right,
}

/*
struct Matrix {
    matrix: Vec<Vec<char>>,
    width: usize,
    height: usize,
    shape: (usize, usize),
}

impl Matrix {
    fn setup(rows: Vec<Vec<char>>) -> Matrix {
        let width = rows[0].len();
        let height = rows.len();
        let shape = (width, height);
        Matrix {
            matrix: rows,
            width,
            height,
            shape,
        }
    }
    fn row(&self, row_idx: usize) -> Vec<char> {
        self.matrix[row_idx].clone()
    }
    fn row_string(&self, idx: usize) -> String {
        let row = &self.row(idx);
        row.iter().collect()
    }
    fn column(&self, col_idx: usize) -> Vec<char> {
        let mut col: Vec<char> = Vec::new();
        for i in self.matrix.iter() {
            col.push(i[col_idx])
        }
        col
    }
    fn col_string(&self, idx: usize) -> String {
        let col = &self.column(idx);
        col.iter().collect()
    }
    fn diagonal(&self, offset: i32, orientation: MatrixDiagonalOrientation) -> Vec<char> {
        let mut diag: Vec<char> = Vec::new();
        let diag_iter_right = 0..cmp::min(self.width, self.height);
        let diag_iter_left = self.width..cmp::min(self.width, self.height);
        match orientation {
            MatrixDiagonalOrientation::Right => match offset.cmp(&0) {
                cmp::Ordering::Equal => {
                    for i in diag_iter_right {
                        diag.push(self.matrix[i][i])
                    }
                }
                cmp::Ordering::Greater => {
                    for i in diag_iter_right {
                        diag.push(self.matrix[i + offset as usize][i])
                    }
                }
                cmp::Ordering::Less => {
                    for i in diag_iter_right {
                        diag.push(self.matrix[i][i + offset as usize])
                    }
                }
            },
            MatrixDiagonalOrientation::Left => diag_iter_left.collect(),
        };
        match offset.cmp(&0) {
            cmp::Ordering::Equal => {
                for i in diag_iter_oriented {
                    diag.push(self.matrix[i][i])
                }
            }
            cmp::Ordering::Greater => {
                for i in diag_iter_oriented {
                    diag.push(self.matrix[i + offset as usize][i])
                }
            }
            cmp::Ordering::Less => {
                for i in diag_iter_oriented {
                    diag.push(self.matrix[i][i + offset as usize])
                }
            }
        }
        diag
    }
}
*/

fn find_char_in_direction(
    matrix: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    seeked_char: char,
    direction: (i32, i32),
) -> bool {
    println!(
        "LETTER IN COORDS: {}",
        matrix[(x + direction.0) as usize][(y + direction.1) as usize]
    );

    let matrix_width = matrix[0].len();
    let matrix_height = matrix.len();
    let other_x = x + direction.0;
    let other_y = y + direction.1;
    if other_x < 0 || other_y < 0 || other_x > matrix_width as i32 || other_y > matrix_height as i32
    {
        return false;
    }
    if matrix[(other_x) as usize][(other_y) as usize] == seeked_char {
        true
    } else {
        false
    }
}

#[derive(Debug)]
struct WordCoordinates {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}
impl WordCoordinates {
    fn print_coords(&self) {
        println!(
            "{};{} - {}.{}",
            self.start_x, self.start_y, self.end_x, self.end_y
        )
    }
}

fn search_xmas_from_pos(matrix: &Vec<Vec<char>>, x: i32, y: i32) -> Vec<WordCoordinates> {
    let matrix_width = matrix[0].len();
    let matrix_height = matrix.len();
    println!("MATRIX SHAPE: {}X{}", matrix_width, matrix_height);
    let mut found_coordinates: Vec<WordCoordinates> = Vec::new();
    for x_iter in -1..2 {
        for y_iter in -1..2 {
            //println!("{} and {}", x_iter, y_iter);

            if x_iter == 0 && y_iter == 0 {
                continue;
            }
            let other_x = x + x_iter;
            let other_y = y + y_iter;
            println!("{} {}", other_x, other_y);
            if other_x >= 0
                && other_y >= 0
                && other_x < matrix_width as i32
                && other_y < matrix_height as i32
            {
                println!(
                    "searching M {} {} from {} {}: {} {}",
                    x_iter, y_iter, x, y, other_x, other_y
                );
                if find_char_in_direction(matrix, x, y, 'M', (x_iter, y_iter)) {
                    println!("FOUND M");

                    println!(
                        "searching M {} {} from {} {}: {} {}",
                        x_iter,
                        y_iter,
                        x,
                        y,
                        other_x + x_iter,
                        other_y + y_iter
                    );
                    if find_char_in_direction(
                        matrix,
                        other_x + x_iter,
                        other_y + x_iter,
                        'A',
                        (x_iter, y_iter),
                    ) {
                        if find_char_in_direction(
                            matrix,
                            other_x + x_iter * 2,
                            other_y + x_iter * 2,
                            'S',
                            (x_iter, y_iter),
                        ) {
                            let tempcoords: WordCoordinates = WordCoordinates {
                                start_x: x,
                                start_y: y,
                                end_x: x + 4,
                                end_y: y + 4,
                            };
                            found_coordinates.push(tempcoords)
                        }
                    }
                }
            }
        }
    }
    found_coordinates
}

fn find_words(
    matrix: Vec<Vec<char>>,
    x: i32,
    y: i32,
    seeked_char: char,
    directions: Vec<i32>,
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
    let input_path = String::from("input/day4/test_input.txt");
    let contents = fs::read_to_string(input_path).expect("Trouble reading file");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for i in contents.lines() {
        matrix.push(i.chars().collect())
    }
    let found = search_xmas_from_pos(&matrix, 1, 1);
    for i in found.iter() {
        i.print_coords()
    }
}
