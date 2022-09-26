use std::{
    io::{stdin, stdout, Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use colored::Colorize;
use rand::{
    prelude::{SliceRandom, StdRng},
    SeedableRng,
};
use wordle::{Game, GameStatus};

const ALL_WORDS: &str = include_str!("words.txt");
const WORD_LENGTH: usize = 5;
const MAX_TRIES: usize = 6;

fn main() {
    let words_list = words_list();
    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs() / 86400,
        Err(_) => panic!("Did we got back in time?"),
    };
    let mut rng = StdRng::seed_from_u64(seed);
    let game = Game::new(
        words_list
            .choose(&mut rng)
            .expect("An error occurred while trying to choose a random word"),
        WORD_LENGTH,
        MAX_TRIES,
    )
    .expect("An error occurred while trying to create new game");
    game_loop(game, &words_list);
    print!("{}", "Press <Enter> to continue...".green());
    stdout().flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn words_list() -> Vec<String> {
    ALL_WORDS
        .split('\n')
        .skip(2)
        .map(sanitize_word)
        .filter(|line| line.len() == WORD_LENGTH)
        .collect()
}

fn game_loop(mut game: Game, words_list: &Vec<String>) {
    // let game = Game::new("testy", WORD_LENGTH, MAX_TRIES)
    //     .expect("An error occurred while trying to create game");
    loop {
        clearscreen::clear().unwrap();
        let result = loop {
            game.display_guesses();
            print!("Enter your guess >> ");
            stdout().flush().unwrap();
            let mut word = String::new();
            stdin().read_line(&mut word).unwrap();
            word = sanitize_word(&word);
            if !validate_word(&word, &words_list) {
                clearscreen::clear().unwrap();
                println!("{}{}", word.red(), " not a valid word".red());
                continue;
            }
            break game
                .add_word(&word)
                .expect("An error occurred while trying to add the word");
        };
        match result.get_game_status() {
            GameStatus::Won => {
                clearscreen::clear().unwrap();
                game.display_guesses();
                println!("{}", "You won the game!".green());
                return;
            }
            GameStatus::Lost => {
                clearscreen::clear().unwrap();
                game.display_guesses();
                println!(
                    "{} The word was {}",
                    "You lost!".red(),
                    game.get_word().to_lowercase().green()
                );
                return;
            }
            GameStatus::Playing => {
                println!()
            }
        }
    }
}

fn validate_word(word: &str, words_list: &Vec<String>) -> bool {
    word.len() == WORD_LENGTH && words_list.contains(&word.to_string())
}

fn sanitize_word(word: &str) -> String {
    word.trim()
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}
