use std::fs;

enum RuleOrder {
    Before,
    After,
}

struct Rule {
    left_number: i32,
    right_number: i32,
}

pub fn complete_day_5(input_path: String) {
    //let input_path = String::from("input/day4/input.txt");
    let contents = fs::read_to_string(input_path).expect("Trouble reading file");
    let mut switch: bool = false;
    let mut rules: Vec<Rule> = Vec::new();
    let mut rules: Vec<Rule> = Vec::new();

    for i in contents.lines() {
        if i.len() == 0 {
            switch = true;
        }
        match switch {
            false => {
                let mut split_rule = i.split("|");
                rules.push(Rule {
                    left_number: split_rule.next().unwrap().parse::<i32>().unwrap(),
                    right_number: split_rule.next().unwrap().parse::<i32>().unwrap(),
                })
            }
            true => {
                let mut split_order = i.split(",");
            }
        }
    }

    println!("{:#?}", contents.lines().next())
}
