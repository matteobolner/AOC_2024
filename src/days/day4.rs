use std::{collections::HashSet, fs};

#[derive(Clone)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn match_direction_with_coords(dir: Directions) -> (i32, i32) {
    match dir {
        Directions::UpLeft => (-1, 1),
        Directions::Up => (0, 1),
        Directions::UpRight => (1, 1),
        Directions::Left => (-1, 0),
        Directions::Right => (1, 0),
        Directions::DownLeft => (-1, -1),
        Directions::Down => (0, -1),
        Directions::DownRight => (1, -1),
    }
}

fn advance_in_direction(dir: (i32, i32), pos: (i32, i32)) -> (i32, i32) {
    (pos.0 + dir.0, pos.1 + dir.1)
}

fn find_char_in_direction(
    matrix: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    seeked_char: char,
    direction: (i32, i32),
) -> bool {
    let matrix_width = matrix[0].len();
    let matrix_height = matrix.len();
    let other_x = x + direction.0;
    let other_y = y + direction.1;
    if other_x < 0
        || other_y < 0
        || other_x > matrix_width as i32 - 1
        || other_y > matrix_height as i32 - 1
    {
        return false;
    }
    //println!("{} {}", other_x, other_y);
    if matrix[(other_x) as usize][(other_y) as usize] == seeked_char {
        /*println!(
            "{} {} -> {}",
            other_x, other_y, matrix[other_x as usize][other_y as usize]
        );*/

        true
    } else {
        false
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
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

fn search_xmas_from_pos(
    matrix: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    mut already_found: HashSet<WordCoordinates>,
) -> HashSet<WordCoordinates> {
    let all_directions = [
        Directions::Up,
        Directions::Down,
        Directions::Left,
        Directions::Right,
        Directions::UpLeft,
        Directions::UpRight,
        Directions::DownLeft,
        Directions::DownRight,
    ];
    for direction in all_directions {
        let direction_coords = match_direction_with_coords(direction);
        if find_char_in_direction(matrix, x, y, 'M', direction_coords) {
            let mut advanced_coords = advance_in_direction(direction_coords, (x, y));
            if find_char_in_direction(
                matrix,
                advanced_coords.0,
                advanced_coords.1,
                'A',
                direction_coords,
            ) {
                advanced_coords = advance_in_direction(direction_coords, advanced_coords);
                if find_char_in_direction(
                    matrix,
                    advanced_coords.0,
                    advanced_coords.1,
                    'S',
                    direction_coords,
                ) {
                    advanced_coords = advance_in_direction(direction_coords, advanced_coords);
                    let tempcoords: WordCoordinates = WordCoordinates {
                        start_x: x,
                        start_y: y,
                        end_x: advanced_coords.0,
                        end_y: advanced_coords.1,
                    };
                    //println!("FOUND XMAS at {:#?}", tempcoords);
                    already_found.insert(tempcoords);
                }
            }
        }
    }
    already_found
}

fn find_mas_in_diag(
    matrix: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    dirs: (Directions, Directions),
) -> bool {
    (find_char_in_direction(
        matrix,
        x,
        y,
        'M',
        match_direction_with_coords(dirs.clone().0),
    ) & find_char_in_direction(
        matrix,
        x,
        y,
        'S',
        match_direction_with_coords(dirs.clone().1),
    )) || (find_char_in_direction(matrix, x, y, 'M', match_direction_with_coords(dirs.1))
        & find_char_in_direction(matrix, x, y, 'S', match_direction_with_coords(dirs.0)))
}

fn search_x_of_mas_from_pos(
    matrix: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    mut already_found: HashSet<(i32, i32)>,
) -> HashSet<(i32, i32)> {
    if find_mas_in_diag(matrix, x, y, (Directions::UpLeft, Directions::DownRight))
        & find_mas_in_diag(matrix, x, y, (Directions::UpRight, Directions::DownLeft))
    {
        already_found.insert((x, y));
    }
    already_found
}

pub fn complete_day_4(input_path: String) {
    //let input_path = String::from("input/day4/input.txt");
    let contents = fs::read_to_string(input_path).expect("Trouble reading file");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for i in contents.lines() {
        matrix.push(i.chars().collect())
    }

    let mut found_coordinates: HashSet<WordCoordinates> = HashSet::new();
    for (x, x_vec) in matrix.iter().enumerate() {
        for (y, pos_char) in x_vec.iter().enumerate() {
            if pos_char.clone() == 'X' {
                found_coordinates =
                    search_xmas_from_pos(&matrix, x as i32, y as i32, found_coordinates);
            }
        }
    }
    println!("N XMAS FOUND: {}", found_coordinates.len());

    let mut found_x_mas_coordinates: HashSet<(i32, i32)> = HashSet::new();
    for (x, x_vec) in matrix.iter().enumerate() {
        for (y, pos_char) in x_vec.iter().enumerate() {
            if pos_char.clone() == 'A' {
                found_x_mas_coordinates =
                    search_x_of_mas_from_pos(&matrix, x as i32, y as i32, found_x_mas_coordinates);
            }
        }
    }
    println!("N X OF MAS FOUND: {}", found_x_mas_coordinates.len());
}
