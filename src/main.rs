use csv::Reader;
use std::error::Error;
use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::process;
use rand::{random, Rng};

fn main() {
    let args: Vec<String> = env::args().collect();

    // todo: declare all paths based on file rather than working directory
    // todo: parse value from args and feed into get_phrase_list
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
        println!("Starting new game");
        let answer = match get_random_phrase(&phrase_list) {
            Some(phrase) => phrase,
            None => {
                // todo: prompt for new filepath instead
                panic!()
            }
        };

        let mut letter_locations: HashMap<char, Vec<u16>> = hashify_answer(&answer);
        let mut guessed_letters: Vec<char> = vec![];

        let mut winner = false;
        while !winner {
            let mut user_input: String = String::new();
            // todo: handle errors gracefully
            io::stdin().read_line(&mut user_input).expect("Failed to read line properly");

            if user_input.ends_with('\n') {
                user_input.pop();
                if user_input.ends_with('\r') {
                    user_input.pop();
                }
            }

            let mut chars: Vec<char> = user_input.chars().collect();

            if chars[0] == '!' {
                match chars[1] {
                    'q' => {
                        new_game = false;
                        break
                    },
                    _ => {}
                }
                continue
            }

            for character in chars.iter() {
                println!("Character {} found", character)
            }

            if user_input.len() > 1 {
                println!("User input was large");
                println!("{}", user_input.len());
                guess_word(&user_input, &answer);
                continue
            }

            let guess = &chars[0];
            let guess_result = guess_letter(&chars[0], &letter_locations);
            match guess_result {
                Some(v) => {guessed_letters.push(*guess); println!("Your guess was correct and appears {v} times")},
                None => {println!("Guess again")}
            }
            // let (letter_found, mut letter_locations) = guess_letter(&chars[0], &answer, letter_locations);

        }
        println!("Your phrase is: {answer} \n")
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
// Consumes a guess, and the hashmap representation of the correct answer
// Returns a vector of the positions that letter is found at in the answer string.
fn guess_letter(guess: &char, letter_locations: &HashMap<char, Vec<u16>>) -> Option<u32> {
    if letter_locations.contains_key(guess) {
        return Some(letter_locations[guess].len() as u32)
    }
    return None
}

fn guess_word(guess: &String, answer: &String) {
    return
}

fn hashify_answer(answer: &String) -> HashMap<char, Vec<u16>> {
    let answer = answer.to_lowercase();
    let mut letter_locations: HashMap<char, Vec<u16>> = HashMap::new();
    let mut position = 0;
    for letter in answer.chars() {
        if letter_locations.contains_key(&letter) {
            letter_locations
                .entry(letter)
                .and_modify(|e| {e.push(position)});
            position += 1;
            continue
        }
        letter_locations.insert(letter, vec![position]);
        position += 1;
    }
    return letter_locations
}

// Possibly rename this, as it doesn't describe the function behavior very well
// Add a part of the crab to noose
fn hang_crustacean_part() {
    return
}