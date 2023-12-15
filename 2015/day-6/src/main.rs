mod input;

use regex::Regex;

enum Action {
    Toggle,
    TurnOff,
    TurnOn,
}

impl Action {
    fn execute(&self, state: &mut i8) {
        match self {
            Action::Toggle => {
                if *state == 0 {
                    *state = 1
                } else {
                    *state = 0
                }
            }
            Action::TurnOff => *state = 0,
            Action::TurnOn => *state = 1,
        };
    }
}

fn main() {
    let row_num = 1000;
    let col_num = 1000;
    let mut lights = vec![vec![0; col_num]; row_num];
    let input: Vec<&str> = input::get_input().split("\n").collect();

    for i in 0..input.len() {
        let action = if input[i].contains("turn off") {
            Action::TurnOff
        } else if input[i].contains("turn on") {
            Action::TurnOn
        } else {
            Action::Toggle
        };

        // extract positions
        let trimmed_input = &input[i]
            .trim()
            .replace("toggle", "")
            .replace("turn off", "")
            .replace("turn on", "")
            .replace("through", "");

        let reg = Regex::new(r"\d+,\d+").unwrap();
        let positions_raw: Vec<&str> = reg
            .find_iter(&trimmed_input)
            .map(|mat| mat.as_str())
            .collect();

        let from: Vec<i32> = positions_raw[0]
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        let to: Vec<i32> = positions_raw[1]
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        do_something(action, &mut lights, from, to);
    }

    let mut count_on: i32 = 0;
    for x in 0..row_num {
        for y in 0..col_num {
            count_on += lights[x][y] as i32;
        }
    }
    println!("{} lights are on.", count_on);
}

fn do_something(action: Action, lights: &mut Vec<Vec<i8>>, from: Vec<i32>, to: Vec<i32>) {
    for x in from[1]..=to[1] {
        for y in from[0]..=to[0] {
            action.execute(&mut lights[x as usize][y as usize]);
        }
    }
}
