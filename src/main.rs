use ansi_term::Colour;
use itertools::Itertools;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use text_io::read;

#[derive(Debug, Copy, Clone, PartialEq)]
enum KeyResult {
    // Use enum to give a name to each state the key can be in
    CorrectPosition,
    IncorrectPosition,
    NotInWord,
}

fn compare(guess: &str, answer: &str) -> [KeyResult; 5] {
    let guess_chars: Vec<char> = guess.chars().collect();
    let mut answer_chars: Vec<char> = answer.chars().collect();
    let mut result = [KeyResult::NotInWord; 5];

    for index in 0..answer.len() {
        if guess_chars[index] == answer_chars[index] {
            result[index] = KeyResult::CorrectPosition;
            answer_chars[index] = '-';
        }
    }

    let mut letter_counts: HashMap<char, u8> = HashMap::new();
    for character in answer_chars.clone().into_iter().unique() {
        if character != '-' {
            letter_counts.insert(
                character,
                answer_chars.iter().filter(|&n| *n == character).count() as u8,
            );
        }
    }

    /*
    This creates a hashmap of all unique characters and how much they occur.
    This is because incorect position characters only show the amount of times there is in the answer.

    Example:
    lever
    knees

    the first e will be IncorrectPosition, second e will be correct position.
    HOWEVER:
    lever
    mover

    the first e will be NotInWord, because the answer does not have 2 e characters.
    */

    for index in 0..answer.len() {
        if answer_chars.contains(&guess_chars[index])
            && letter_counts[&guess_chars[index]] > 0
            && result[index] != KeyResult::CorrectPosition
        {
            letter_counts.insert(guess_chars[index], letter_counts[&guess_chars[index]] - 1);
            result[index] = KeyResult::IncorrectPosition;
        }
    }

    result
}

fn print_guess(guess: &str, result: [KeyResult; 5]) {
    for (index, character) in guess.chars().enumerate() {
        print!(
            "{}",
            match result[index] {
                KeyResult::CorrectPosition => Colour::Green.paint(character.to_string()),
                KeyResult::IncorrectPosition => Colour::Yellow.paint(character.to_string()),
                KeyResult::NotInWord => Colour::White.paint(character.to_string()),
            }
        );
    }
    println!();
}

fn print_title() {
    println!("                             __  .__          ");
    println!("  ___________ __ __  _______/  |_|  |   ____  ");
    println!("_/ ___\\_  __ \\  |  \\/  ___/\\   __\\  | _/ __ \\ ");
    println!("\\  \\___|  | \\/  |  /\\___ \\  |  | |  |_\\  ___/ ");
    println!(" \\___  >__|  |____//____  > |__| |____/\\___  >");
    println!("     \\/                 \\/                 \\/ ");
    println!("Wordle... in rust!");
}

fn clear() {
    // Clears the screen, wrapped in a function because It is unclear
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    //println!();
}

fn instructions() {
    println!();
    println!(
        "Enter 5 letters words, the color the letters are repeated to you give you information."
    );
    println!(
        "{} tells you that you've guessed the right letter for that position.",
        Colour::Green.paint("Green")
    );
    println!(
        "{} tells you that the letter is in the word, just not that position.",
        Colour::Yellow.paint("Yellow")
    );
    println!(
        "{} tells you it is not in the word",
        Colour::White.paint("White")
    );
    println!();
    println!("Use this information to guess the word!");
    println!();
}

fn choice_text() {
    println!("1. Play Again");
    println!("2. Display Instructions");
    println!("3. Quit");
}

fn get_guess(
    guess: &str,
    vaild_words: &HashSet<&str>,
    previous_guesses: &Vec<(String, [KeyResult; 5])>,
    word: &str,
) -> bool {
    clear();
    match (guess.len(), vaild_words.contains(guess)) {
        (length, _) if length != 5 => {
            println!("Incorrect word length");
            false
        }
        (_, vaild) if !vaild => {
            println!("Not a vaild word");
            false
        }
        (_, _) if previous_guesses.contains(&(guess.to_string(), compare(guess, word))) => {
            println!("You've already guessed this word!");
            false
        }
        (_, _) => true,
    }
}

fn main() {
    let vaild_words: HashSet<&str> = include_str!("../possible_words.txt").lines().collect(); // Vaild words, you must guess a word in this list
    let possible_answers: Vec<&str> = include_str!("../possible_answers.txt").lines().collect(); // Possible answers, these are more normal words

    // include_str includes the string into the binary, this means the word lists won't need to exist when compiled

    let mut rng = rand::thread_rng();

    'outer: loop {
        let word: String = possible_answers.choose(&mut rng).unwrap().to_string();
        let mut guesses = 0;
        let mut previous_guesses: Vec<(String, [KeyResult; 5])> = Vec::new();

        clear();
        print_title();
        instructions();

        loop {
            for entry in &previous_guesses {
                print_guess(&entry.0, entry.1)
            }
            let guess: String = read!("{}\n");
            if !get_guess(&guess, &vaild_words, &previous_guesses, &word) {
                continue;
            }

            guesses += 1;
            if guess == word {
                loop {
                    clear();
                    for entry in &previous_guesses {
                        print_guess(&entry.0, entry.1)
                    }
                    print_guess(&guess, [KeyResult::CorrectPosition; 5]);
                    println!("You've won! Guesses: {}!", guesses);
                    println!("Press enter to continue");
                    println!();
                    choice_text();
                    let choice: String = read!("{}\n");
                    match choice.as_str() {
                        "1" => continue 'outer,
                        "2" => {
                            clear();
                            instructions();

                            println!("Press enter to continue");
                            let _: String = read!("{}\n");
                        }
                        "3" => break 'outer,
                        _ => {}
                    }
                }
            }

            previous_guesses.push((guess.clone(), compare(&guess, &word)));
        }
    }
}
