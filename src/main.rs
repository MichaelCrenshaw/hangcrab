use std::error::Error;
use std::io;
use std::collections::HashMap;
use std::fs::File;
use std::process::exit;
use rand::Rng;

fn main() {
    let passphrase_file_path = "wordlist.csv";

    let phrase_list = match get_phrase_list(passphrase_file_path) {
        Ok(phrases) => phrases,
        Err(error) => {
            print!("{error}");
            vec![String::from("Artie's"), String::from("We have the beets")]
        }
    };

    println!("Please guess a letter, use !g to guess the phrase, or use !q to quit");

    let mut new_game = true;
    while new_game {
        println!("Starting new game");
        let answer = match get_random_phrase(&phrase_list) {
            Some(phrase) => phrase,
            None => {"Word file not found, please check your executable's directory."; exit(1)}
        };

        let mut lives = 5;
        let letter_locations: HashMap<char, Vec<u16>> = hashify_answer(&answer);
        let mut guessed_letters: Vec<char> = vec![];

        let mut winner = false;
        while !winner {
            if lives == 0 {
                println!("You've run out of guesses, your phrase was: {}", answer);
                break
            }


            let mut user_input: String = String::new();
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {},
                Err(_) => {println!("Couldn't read line, please try again"); continue}
            }

            // remove newline characters from user input
            if user_input.ends_with('\n') {
                user_input.pop();
                if user_input.ends_with('\r') {
                    user_input.pop();
                }
            }

            if user_input.len() < 1 {
                println!("Please enter a guess or command");
                continue
            }

            let chars: Vec<char> = user_input.chars().collect();

            if chars[0] == '!' {
                match chars[1] {
                    'q' | 'Q' => {
                        new_game = false;
                        break
                    },
                    'g' | 'G' => {
                        if guess_word(&String::from_iter(chars.iter())[3..], &answer[0..]) {
                            winner = true;
                            continue
                        } else {
                            lives -= 1;
                            continue
                        }
                    },
                    _ => {}
                }
                continue
            }

            let guess = &chars[0];

            if guessed_letters.contains(&guess.to_lowercase().collect::<Vec<_>>()[0]) {
                println!("You've already guessed {}", guess);
                continue
            }


            let guess_result = guess_letter(&chars[0], &letter_locations);
            match guess_result {
                Some(v) => {
                    guessed_letters.push(*&guess.to_lowercase().collect::<Vec<_>>()[0]);
                    println!("Your guess was correct and appears {v} times")
                },
                None => {
                    lives -= 1;
                    let pluralized_word = if lives > 1 {"lives"} else {"life"};
                    println!("Your letter was not in the phrase, you have {} {} left", lives, pluralized_word);
                }
            }

            if check_victory(&answer, &guessed_letters) {
                winner = true;
                continue
            }

            println!("{}\n", get_word_progress(&answer, &guessed_letters))
        }
    }
    println!("Thanks for playing hangcrab");
    exit(0);
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
    return Ok(phrases)
}

fn get_random_phrase(word_list: &Vec<String>) -> Option<String> {
    let random_int = rand::thread_rng().gen_range(0..word_list.len());
    let phrase = word_list.get(random_int)?;
    return Some(phrase.to_owned())
}

// Consumes a guess, and the hashmap representation of the correct answer
// Returns a vector of the positions that letter is found at in the answer string.
fn guess_letter(guess: &char, letter_locations: &HashMap<char, Vec<u16>>) -> Option<u32> {
    if letter_locations.contains_key(guess) {
        return Some(letter_locations[guess].len() as u32);
    }
    return None
}

fn guess_word(guess: &str, answer: &str) -> bool {
    if guess.to_lowercase() == answer.to_lowercase() {
        println!("You guessed the correct phrase!");
        return true;
    }
    println!("{} isn't equal to {}", guess, answer);
    false
}

// This could be done with a vector instead, since we handle this type of logic with vectors later in the code anyways
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

fn get_word_progress(answer: &String, guessed_letters: &Vec<char>) -> String {
    let chars = answer.chars();
    let mut result = String::new();
    for ch in chars {
        match ch {
            'a'..='z' | 'A'..='Z' => {
                if guessed_letters.contains(&ch.to_lowercase().collect::<Vec<_>>()[0]) {
                    result.push(ch);
                } else {
                    result.push('_')
                }
            },
            '\'' | ' ' | '-' => {
                result.push(ch);
            },
            _ => {}
        }
    }
    return result.to_owned();
}

fn check_victory(answer: &String, guessed_letters: &Vec<char>) -> bool {
    let ans = &answer.replace(&['\'', '_', ' ', '-'][..], "");

    let chars = ans.chars();
    for ch in chars {
        if guessed_letters.contains(&ch.to_lowercase().collect::<Vec<_>>()[0]) {
            continue
        }
        return false
    }

    return true
}