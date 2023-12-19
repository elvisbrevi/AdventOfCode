mod input;
use std::collections::HashMap;

enum BitwiseOperation {
    VAL(u16),
    AND(u16, u16),
    OR(u16, u16),
    LSHIFT(u16, u16),
    RSHIFT(u16, u16),
    NOT(u16),
}

impl BitwiseOperation {
    fn evaluate(&self) -> u16 {
        match self {
            BitwiseOperation::VAL(a) => *a,
            BitwiseOperation::AND(a, b) => a & b,
            BitwiseOperation::OR(a, b) => a | b,
            BitwiseOperation::LSHIFT(a, b) => a << b,
            BitwiseOperation::RSHIFT(a, b) => a >> b,
            BitwiseOperation::NOT(a) => !a,
        }
    }
}

fn main() {
    let input = input::get_test_input();
    let mut operations: HashMap<&str, Vec<&str>> = HashMap::new();
    let binding = input.replace("->", "");

    // reading input
    for operation_string in binding.split("\n") {
        // divide usefull words in a vector
        let mut words: Vec<&str> = operation_string.trim().split(" ").collect();
        // get the character identifier for wire
        let key = words.swap_remove(words.len() - 1);
        // remove las element '->'
        words.remove(words.len() - 1);

        // save wire and it's bitwise operation in a hasmap
        operations.insert(key, words);
    }

    //excute operation
    let result = find_value("x", &operations);
    println!("{}", result)
}

fn is_numeric(value: &str) -> bool {
    for v in value.chars() {
        if !v.is_numeric() {
            return false;
        }
    }
    true
}

fn find_value(key: &str, operations: &HashMap<&str, Vec<&str>>) -> u16 {
    if let Some(op) = &operations.get(key) {
        match op.as_slice() {
            [value] => {
                if is_numeric(value) {
                    value.parse::<u16>().unwrap()
                } else {
                    find_value(value, operations)
                }
            }
            ["NOT", value] => {
                let value_u16 = if is_numeric(value) {
                    value.parse::<u16>().unwrap()
                } else {
                    find_value(value, operations)
                };
                !value_u16
            }
            _ => 0, //BitwiseOperation::VAL(0).evaluate(),
        }
    } else {
        0
    }

    // bitwise operation
    /* let operation = match operations.get(key).as_slice() {
        [value] => BitwiseOperation::NONE(value.parse::<u16>().expect("Conversion error")),
        ["NOT", value] => BitwiseOperation::NOT(value.parse::<u16>().expect("Conversion error")),
        [op, left, right] => {
            let left_val = left.parse::<u16>().unwrap();
            let right_val = right.parse::<u16>().unwrap();
            match *op {
                "AND" => BitwiseOperation::AND(left_val, right_val),
                "OR" => BitwiseOperation::OR(left_val, right_val),
                "LSHIFT" => BitwiseOperation::LSHIFT(left_val, right_val),
                "RSHIFT" => BitwiseOperation::RSHIFT(left_val, right_val),
                _ => panic!("unknow operation"),
            }
        }
        _ => panic!("Invalid format"),
    }; */
}
