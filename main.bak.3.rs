use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <sentence>", args[0]);
        std::process::exit(1);
    }

    let sentence = args[1..].join(" ");
    let spoonerized = spoonerize(&sentence);
    println!("{}", spoonerized);
}

fn spoonerize(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut new_words = Vec::new();
    let mut i = 0;

    while i < words.len() {
        if words[i].len() <= 3 {
            new_words.push(words[i].to_string());
            i += 1;
            continue;
        }

        if i + 1 < words.len() && words[i + 1].len() > 3 {
            let (new_first, new_second) = swap_initial_consonants(words[i], words[i + 1]);
            new_words.push(capitalize_like(new_first, words[i]));
            new_words.push(capitalize_like(new_second, words[i + 1]));
            i += 2; // Skip the next word as it has been processed
        } else {
            new_words.push(words[i].to_string());
            i += 1;
        }
    }

    new_words.join(" ")
}

fn swap_initial_consonants(first: &str, second: &str) -> (String, String) {
    let re = Regex::new(r"^[^aeiouAEIOU]*").unwrap();
    let first_consonants = re.find(first).unwrap().as_str();
    let second_consonants = re.find(second).unwrap().as_str();

    let spoonerized_first = format!("{}{}", second_consonants, &first[first_consonants.len()..]).to_lowercase();
    let spoonerized_second = format!("{}{}", first_consonants, &second[second_consonants.len()..]).to_lowercase();

    (spoonerized_first, spoonerized_second)
}

// Function to capitalize a new word based on the pattern of the original word
fn capitalize_like(mut new_word: String, original_word: &str) -> String {
    let mut new_chars = new_word.chars().collect::<Vec<_>>();
    let original_chars = original_word.chars().collect::<Vec<_>>();

    for (i, original_char) in original_chars.iter().enumerate() {
        if i < new_chars.len() {
            new_chars[i] = if original_char.is_uppercase() {
                new_chars[i].to_uppercase().next().unwrap()
            } else {
                new_chars[i].to_lowercase().next().unwrap()
            };
        }
    }

    new_chars.into_iter().collect()
}

