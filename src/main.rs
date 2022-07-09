use csv::Reader;
use std::error::Error;
use std::{env, io};
use std::fs::File;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // todo: parse value from args and feed into gen_word
    let words = get_words();
    let words = match words {
        Ok(words) => words,
        // error handling could be more robust here
        Err(error) => vec![String::from("taco"), String::from("additional words")]
    };
    // testcode
    for line in words{
        print!("{line}");
    }

    let word = gen_word();
    println!("{}", word)
}

fn get_words() -> Result<Vec<String>, Box<dyn Error>>{
    let file_path = "../wordlist.csv";
    let file = File::open(file_path)?;
    let mut reader = csv::ReaderBuilder::new()
        .from_reader(file);

    let mut words = Vec::new();
    for result in reader.records() {
        for row in result.iter() {
            // TODO: this code is unsafe, remove after testing
            let mut row = row.as_slice();
            words.push(String::from(row));
        }
    }
    Ok(words)
}

fn gen_word() -> String {
    let words: String = String::new();

    return words
}

fn guess_letter(guess: char, answer: String, guesses: Vec<String>) {
    return
}

fn guess_word(guess: String, answer: String, guesses: Vec<String>) {
    return
}

// Possibly rename this, as it doesn't describe the function behavior very well
// Add a part of the crab to noose
fn hang_crustacean_part() {
    return
}