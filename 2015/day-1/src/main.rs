use std::collections::HashMap;

use crate::input::get_input;
mod input;

fn main() {
    let posible_values = HashMap::from([('(', 1), (')', -1)]);
    let input = get_input();
    let mut floor = 0;
    let mut index = 1;
    let mut character_position = 0;

    for char in input.chars() {
        if let Some(value) = posible_values.get(&char) {
            floor += value;
        }

        if character_position == 0 && floor == -1 {
            character_position = index;
        }

        index += 1;
    }

    println!(
        "The position of the character that causes Santa to first enter the basement is {} 🎅",
        character_position
    );
    println!("The instructions take Santa to floor {} 🏢", floor);
}
