mod input;
use std::collections::HashMap;

fn main() {
    let input = input::get_input();
    let mut operations: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut operations_solved: HashMap<&str, u16> = HashMap::new();
    let binding = input.replace("->", "");

    // reading input
    for operation_string in binding.split("\n") {
        // divide usefull words in a vector
        let mut words: Vec<&str> = operation_string.trim().split(" ").collect();
        // get the character identifier for wire
        let key = words.pop().expect("No key found");
        // remove las element '->'
        words.pop();

        // save wire and it's bitwise operation in a hasmap
        operations.insert(key, words);
    }

    //excute operation
    let result = find_value("a", &operations, &mut operations_solved);
    println!("{}", result)
}

fn is_numeric(value: &str) -> bool {
    value.chars().all(char::is_numeric)
}

fn find_value<'a>(
    key: &'a str,
    operations: &'a HashMap<&str, Vec<&str>>,
    operations_solved: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Some(op) = operations_solved.get(key) {
        return *op;
    }

    if let Some(op) = &operations.get(key) {
        let result = match op.as_slice() {
            [value] => {
                if is_numeric(value) {
                    value.parse::<u16>().expect("conversion error")
                } else {
                    find_value(value, operations, operations_solved)
                }
            }
            ["NOT", value] => {
                let value_u16 = if is_numeric(value) {
                    value.parse::<u16>().expect("conversion error")
                } else {
                    find_value(value, operations, operations_solved)
                };
                !value_u16
            }
            [left_value, op, right_value] => {
                let left_value_u16 = if is_numeric(left_value) {
                    left_value.parse::<u16>().expect("conversion error")
                } else {
                    find_value(left_value, operations, operations_solved)
                };

                let right_value_u16 = if is_numeric(right_value) {
                    right_value.parse::<u16>().expect("conversion error")
                } else {
                    find_value(right_value, operations, operations_solved)
                };

                match *op {
                    "AND" => left_value_u16 & right_value_u16,
                    "OR" => left_value_u16 | right_value_u16,
                    "RSHIFT" => left_value_u16 >> right_value_u16,
                    "LSHIFT" => left_value_u16 << right_value_u16,
                    _ => {
                        println!("not found");
                        0
                    }
                }
            }
            _ => {
                println!("not found");
                0
            }
        };

        operations_solved.insert(key, result);
        result
    } else {
        println!("not found");
        0
    }
}
