mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
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

fn day4() {
    let day_4_input = String::from("input/day4/input.txt");
    days::day4::complete_day_4(day_4_input)
}

fn day5() {
    let day_5_input = String::from("input/day5/test_input.txt");
    days::day5::complete_day_5(day_5_input)
}
fn main() {
    day5()
}
