mod input;

fn main() {
    let input: Vec<&str> = input::input().split('\n').collect();

    let mut nice_count = 0;
    for i in &input {
        let input_trimmed = i.trim();
        if is_nice_part_one(input_trimmed) {
            nice_count += 1
        }
    }

    println!("There are {} nice strings for part one", nice_count);

    nice_count = 0;
    for i in &input {
        let input_trimmed = i.trim();
        if is_nice_part_two(input_trimmed) {
            nice_count += 1
        }
    }

    println!("There are {} nice strings for part two", nice_count);
}

fn is_nice_part_two(word: &str) -> bool {
    return have_pair_twice(word) && have_letter_repeat(word);
}

fn have_letter_repeat(word: &str) -> bool {
    for i in 2..word.chars().count() {
        let current_char = word.chars().nth(i).unwrap_or(' ');
        let previous_char = word.chars().nth((i as u32 - 2) as usize).unwrap_or(' ');

        if current_char == previous_char {
            return true;
        }
    }

    false
}

fn have_pair_twice(word: &str) -> bool {
    let mut pair_list: Vec<(u32, String)> = vec![(0, String::new())];

    for i in 1..word.chars().count() {
        let current_char = word.chars().nth(i).unwrap_or(' ');
        let previous_char = word.chars().nth((i as u32 - 1) as usize).unwrap_or(' ');

        let new_pair = format!("{}{}", previous_char, current_char);

        if pair_list
            .iter()
            .any(|x| x.1 == new_pair && x.0 != (i - 1) as u32)
        {
            return true;
        }

        pair_list.push((i as u32, new_pair));
    }

    false
}

fn is_nice_part_one(word: &str) -> bool {
    return has_three_vowels(word) && letter_appears_twice(word) && not_contain_strings(word);
}

fn not_contain_strings(word: &str) -> bool {
    let bad_strings = vec!["ab", "cd", "pq", "xy"];

    return !bad_strings.iter().any(|i| word.contains(i));
}

fn letter_appears_twice(word: &str) -> bool {
    let mut last_letter = ' ';
    let mut twice = false;

    for c in word.chars() {
        if c == last_letter {
            twice = true
        }
        last_letter = c
    }

    twice
}

fn has_three_vowels(word: &str) -> bool {
    let mut vowels_count = 0;
    let vowels = "aeiou";

    for c in word.chars() {
        if vowels.contains(c) {
            vowels_count += 1
        }
    }

    vowels_count > 2
}
