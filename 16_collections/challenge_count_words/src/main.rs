use std::collections::HashMap;
use std::{env, fs, io};

fn main() {
    let file_contents = read_file();
    let mut word_counts = HashMap::new();
    for word in file_contents.split_whitespace() {
        let current_word = word_counts.entry(word).or_insert(0);
        *current_word += 1;
    }

    let mut max_count: u32 = 0;
    let mut max_occurring_words: Vec<&str> = Vec::new();

    for (key, val) in &word_counts {
        // println!("key: {key} val: {val}");
        if *val > max_count {
            max_count = *val;
            max_occurring_words.clear();
            max_occurring_words.push(*key);
        } else if *val == max_count {
            max_occurring_words.push(*key);
        }
    }

    print!("The word(s) with the highest number of occurrences: ");
    for word in max_occurring_words {
        print!("'{word}' ");
    }
    println!("which occurs {max_count} times");
}

fn read_file() -> String {
    assert!(
        (env::args().len() > 1),
        "A filename argument with words to count must be supplied to run this program"
    );

    let filename = env::args().nth(1).unwrap();
    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                panic!("Error! File not found!");
            }
            io::ErrorKind::PermissionDenied => {
                panic!("Error! Permission denied while trying to open the file. Exiting!");
            }
            _ => panic!(
                "Unknown error: {:?} encountered whilst trying to open the file",
                error
            ),
        },
    };
    file_contents.to_lowercase()
}
