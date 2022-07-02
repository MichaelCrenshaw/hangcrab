use std::env;
use std::io::{stdin, stdout, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // todo: parse value from args and feed into gen_word
    let word: String = gen_word(1);
}

fn gen_word(word_count: u32) -> String {
    return String("hi")
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