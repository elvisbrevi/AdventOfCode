use regex::Regex;

fn main() {
    let contents = include_str!("../input.txt");
    let lines: Vec<&str> = contents.split_whitespace().collect();
    let mut literal_strings = 0;
    let mut memory_strings = 0;
    let mut encoded_strings = 0;

    let regex = Regex::new(r"\\x[a-f0-9A-F]{2}").unwrap();

    for line in lines {
        literal_strings += line.len();

        memory_strings += regex
            .replace_all(line, "x")
            .replace("\\\\", "x")
            .replace("\\\"", "x")
            .len()
            - 2;

        encoded_strings += &line.replace("\\", "xx").replace("\"", "xx").len() + 2;
    }

    println!("{} literal_strings", literal_strings);
    println!("{} memory_strings", memory_strings);
    println!(
        "{} literal_strings - memory_strings",
        literal_strings - memory_strings
    );
    println!("{} encoded_strings", encoded_strings);
    println!(
        "{} encoded_strings - literal_strings",
        encoded_strings - literal_strings
    );
}
