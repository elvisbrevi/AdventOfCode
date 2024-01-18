use regex::Regex;

fn main() {
    let contents = include_str!("../input.txt");
    let lines: Vec<&str> = contents.split_whitespace().collect();
    let mut char_string_literals = 0;
    let mut char_in_memory = 0;

    let regex = Regex::new(r"\\x[a-f0-9A-F]{2}").unwrap();

    for line in lines {
        char_string_literals += line.len();

        char_in_memory += regex
            .replace_all(line, "x")
            .replace("\\\\", "x")
            .replace("\\\"", "x")
            .len()
            - 2;
    }

    println!("{}", char_string_literals);
    println!("{}", char_in_memory);
    println!("{}", char_string_literals - char_in_memory);

    // ""           2       0
    // "abc"        5       3
    // "aaa\"aaa"   10      7
    // "\x27"       6       1
}
