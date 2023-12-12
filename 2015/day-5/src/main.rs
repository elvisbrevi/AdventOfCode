mod input;

fn main() {
    let input: Vec<&str> = input::input().split('\n').collect();
    let mut nice_count = 0;
    for i in input {
        let input_trimmed = i.trim();
        if is_nice(input_trimmed) {
            nice_count += 1
        }
    }
    println!("There are {} nice strings", nice_count)
}

fn is_nice(word: &str) -> bool {
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
