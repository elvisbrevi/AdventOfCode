mod input;
use std::collections::{binary_heap, HashMap};

enum BitwiseOperation {
    NONE(u16),
    AND(u16, u16),
    OR(u16, u16),
    LSHIFT(u16, u16),
    RSHIFT(u16, u16),
    NOT(u16),
}

impl BitwiseOperation {
    fn result(&self) -> u16 {
        match self {
            BitwiseOperation::NONE(a) => *a,
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

    for operation_string in input.replace("->", "").split("\n") {
        // divide usefull words in a vector
        let mut words: Vec<&str> = operation_string.trim().split(" ").collect();
        // get the character identifier for wire
        let key = words.swap_remove(words.len() - 1);
        // remove las element '->'
        words.remove(words.len() - 1);

        // save wire and it's bitwise operation in a hasmap
        operations.insert(key, words);
    }
}

fn find_value(key: &str, operations: HashMap<&str, Vec<&str>>) -> u16 {

    if let Some() operations.get(key).
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
