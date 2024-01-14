use std::fs;

fn main() {
    let contents = fs::read_to_string("input-test.txt").unwrap();
    let lines: Vec<&str> = contents.split_whitespace().collect();
    let mut char_string_literals = 0;
    let mut char_in_memory = 0;

    for line in lines {
        char_string_literals += line.len();

        let mut memory = format!("{:0}", &line[1..line.len() - 1])
            .replace("\\\"", "\"")
            .replace("\\\\", "\\");

        // when is a character
        let memory_clone = memory.clone();
        let matches: Vec<&str> = memory_clone.matches("\\x").collect();

        for char in matches {
            let hex_value_str = &memory.find("\\x"); //&memory[index + 2..index + 4];
            if let Ok(hex_value) = u8::from_str_radix(hex_value_str, 16) {
                if let Some(character) = char::from_u32(hex_value as u32) {
                    memory = String::from(
                        &memory
                            .replace(&memory[index..index + 4], String::from(character).as_str()),
                    );

                    println!("memory: {}", memory);
                    println!("memory clone: {}", memory_clone);
                    // println!("valor: {} a", memory);
                    // println!("result: {}", memory.len());
                }
            }
        }

        char_in_memory += memory.len();
    }

    println!("{}", char_string_literals);
    println!("{}", char_in_memory);
    println!("{}", char_string_literals - char_in_memory);

    // ""           2       0
    // "abc"        5       3
    // "aaa\"aaa"   10      7
    // "\x27"       6       1
}
