const VOWELS: [char; 10] = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

enum AsciiCharType {
    AsciiVowel(char),
    AsciiConsonant(char),
    NonAscii(char),
}

use AsciiCharType::AsciiVowel;
use AsciiCharType::AsciiConsonant;
use AsciiCharType::NonAscii;

use std::io::stdin;

fn main() {
    let mut sentence = String::new();

    println!("Please enter a sentence (put spaces after apostrophes):");

    stdin().read_line(&mut sentence)
        .expect("Cannot read line!");

    pig_latin(&mut sentence);

    println!("The pig-latin modified sentence is {}", sentence);
}

// Example "Imagine, all (the) people" becomes "Imagine-hay, all-hay (he-tay) eople-pay"
fn pig_latin(sentence: &mut String) {
    let mut cleared = false;
    let sentence_copy: String = sentence.clone();

    for word in sentence_copy.split_whitespace() {
        let word = String::from(word);
        let mut new_word = String::new();
        let mut new_word_suffix = String::new();
        let mut suffix_started = false;
        let mut first_found = false;
        let mut starts_with: Option<AsciiCharType> = None;

        for ch in word.chars() {
            let is_alphabetic = ch.is_alphabetic();
            if is_alphabetic && !first_found {
                // handle first alphabetic char in string after non-alphabetic prefix
                first_found = true;
                if ch.is_ascii() {
                    if VOWELS.contains(&ch) {
                        starts_with = Some(AsciiVowel(ch));
                        new_word.push(ch);
                    } else {
                        starts_with = Some(AsciiConsonant(ch));
                    }
                } else {
                    starts_with = Some(NonAscii(ch));
                }
            } else if suffix_started || first_found && !is_alphabetic {
                // handle suffix after word
                suffix_started = true;
                new_word_suffix.push(ch);
            } else {
                // either is prefix or middle of word
                new_word.push(ch);
            }
        }

        match starts_with {
            Some(AsciiVowel(_)) => {
                new_word.push_str("-hay");
            },
            Some(AsciiConsonant(ch)) => {
                new_word.push('-');
                new_word.push(ch);
                new_word.push_str("ay");
            },
            _ => {
                // Ignore None and Some(NonAscii(_)) cases
            },
        }

        new_word.push_str(&new_word_suffix);

        if !cleared {
            cleared = true;
            sentence.clear();
        } else {
            sentence.push(' ');
        }

        sentence.push_str(&new_word);
    }
}
