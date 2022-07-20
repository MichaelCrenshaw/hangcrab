use csv::Reader;
use std::error::Error;
use std::{env, io};
use std::fs::File;
use std::process;
use rand::{random, Rng};

fn main() {
    let args: Vec<String> = env::args().collect();

    // todo: declare all paths based on file rather than working directory
    let passphrase_file_path = "wordlist.csv";

    let phrase_list = match get_phrase_list(passphrase_file_path) {
        Ok(phrases) => phrases,
        Err(error) => {
            print!("{error}");
            vec![String::from("Artie's"), String::from("We have the beets")]
        }
    };

    let mut new_game = true;
    while new_game{
        // todo: parse value from args and feed into gen_word
        let word = match get_random_phrase(&phrase_list) {
            Some(phrase) => phrase,
            None => {
                // todo: prompt for new filepath instead
                panic!()
            }
        };
        let mut winner = false;
        while !winner {
        //     let mut guess: String;
        //     // todo: handle errors gracefully
        //     io::stdin().read_line(&mut guess).expect("Failed to read line properly");
        //
        //     let chars: Vec<char> = guess.chars().collect();
        //
        //     if guess.len() == 1 {
        //
        //     }
        //
        //
        }
        print!("Your phrase is: {word}")
    }
    println!("Thanks for playing hangcrab");
    process::exit(0);
}

fn get_phrase_list(file_path: &str) -> Result<Vec<String>, Box<dyn Error>>{
    let file = File::open(file_path)?;
    let mut reader = csv::ReaderBuilder::new()
        .from_reader(file);

    let mut phrases = Vec::new();
    for result in reader.records() {
        for row in result.iter() {
            let row = row.as_slice();
            phrases.push(String::from(row));
        }
    }
    Ok(phrases)
}

fn get_random_phrase(word_list: &Vec<String>) -> Option<String> {
    let random_int = rand::thread_rng().gen_range(0..word_list.len());
    let phrase = word_list.get(random_int)?;
    Some(phrase.to_owned())
}

// todo: disable guessing the same letter twice
fn guess_letter(guess: &char, answer: &String) {
    return
}

fn guess_word(guess: &String, answer: &String) {
    return
}

// Possibly rename this, as it doesn't describe the function behavior very well
// Add a part of the crab to noose
fn hang_crustacean_part() {
    return
}

struct HangcrabPhrase {

}