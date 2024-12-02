mod days {
    pub mod day1;
    pub mod day2;
}
fn day1() {
    let day_1_input = String::from("input/day1/input.txt");
    days::day1::complete_day_1(day_1_input)
}
fn day2() {
    let day_2_input = String::from("input/day2/input.txt");
    days::day2::complete_day_2(day_2_input)
}

fn main() {
    day2()
}
