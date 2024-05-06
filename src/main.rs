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
    let mut processed_words = vec![None; words.len()]; // Placeholder for new words
    let mut big_words = Vec::new(); // Collect indices of big words

    for (i, word) in words.iter().enumerate() {
        if word.len() > 3 {
            big_words.push(i);
        }
    }

    let mut i = 0;
    while i < big_words.len() {
        if i + 1 < big_words.len() {
            let (new_first, new_second) = swap_initial_consonants(words[big_words[i]], words[big_words[i+1]]);
            processed_words[big_words[i]] = Some(capitalize_like(new_first, words[big_words[i]]));
            processed_words[big_words[i+1]] = Some(capitalize_like(new_second, words[big_words[i+1]]));
            i += 2; // Move to the next pair
        } else {
            processed_words[big_words[i]] = Some(words[big_words[i]].to_string());
            i += 1;
        }
    }

    // Fill in the small words
    for (i, word) in words.iter().enumerate() {
        if processed_words[i].is_none() {
            processed_words[i] = Some(word.to_string());
        }
    }

    processed_words.iter().map(|w| w.as_ref().unwrap().as_str()).collect::<Vec<&str>>().join(" ")
}

fn swap_initial_consonants(first: &str, second: &str) -> (String, String) {
    let re = Regex::new(r"^[^aeiouAEIOU]*").unwrap();
    let first_consonants = re.find(first).unwrap().as_str();
    let second_consonants = re.find(second).unwrap().as_str();

    let spoonerized_first = format!("{}{}", second_consonants, &first[first_consonants.len()..]);
    let spoonerized_second = format!("{}{}", first_consonants, &second[second_consonants.len()..]);

    (spoonerized_first, spoonerized_second)
}

// Function to capitalize a new word based on the pattern of the original word
fn capitalize_like(new_word: String, original_word: &str) -> String {
    let original_chars = original_word.chars().collect::<Vec<_>>();
    let mut new_chars = new_word.chars().collect::<Vec<_>>();

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

