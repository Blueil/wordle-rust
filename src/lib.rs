use colored::Colorize;

pub struct Game {
    selected_word: String,
    word_list: Vec<String>,
    max_tries: usize,
    tries: Vec<Vec<Letter>>,
    word_length: usize,
}

impl Game {
    pub fn new(selected_word: &str, word_length: usize, max_tries: usize) -> Result<Game, String> {
        if selected_word.len() != word_length {
            return Err(format!("Word length must be {}!", word_length).to_string());
        }
        Ok(Game {
            selected_word: selected_word.to_string(),
            word_list: Vec::new(),
            tries: Vec::new(),
            word_length,
            max_tries,
        })
    }

    pub fn get_word(&self) -> &str {
        &self.selected_word
    }

    pub fn get_word_list(&self) -> &Vec<String> {
        &self.word_list
    }

    pub fn get_tries(&self) -> &Vec<Vec<Letter>> {
        &self.tries
    }

    pub fn get_max_tries(&self) -> usize {
        self.max_tries
    }

    pub fn get_word_length(&self) -> usize {
        self.word_length
    }

    pub fn display_guesses(&self) {
        for i in 0..self.get_max_tries() {
            if i >= self.get_tries().len() {
                for _ in 0..self.get_word_length() {
                    print!("[ ]");
                }
                println!();
                continue;
            }
            for l in self.get_tries()[i].iter() {
                match l {
                    Letter::Right(c) => print!("[{}]", c.to_string().green()),
                    Letter::WrongPlace(c) => print!("[{}]", c.to_string().yellow()),
                    Letter::Wrong(c) => print!("[{}]", c.to_string().red()),
                }
            }
            println!();
        }
    }

    pub fn add_word(&mut self, word: &str) -> Result<TryResult, String> {
        if word.len() != self.get_word_length() {
            return Err(format!("Word length must be {}!", self.get_word_length()));
        }
        if self.get_word_list().len() >= self.get_max_tries() {
            return Err(format!(
                "Already tried {} times to guess the word, should create another instance of Game!",
                self.get_max_tries()
            ));
        }
        let mut result = Vec::new();
        for (i, c) in word.chars().enumerate() {
            if c == self.get_word().chars().nth(i).unwrap() {
                result.push(Letter::Right(c));
            } else if self.get_word().contains(c) {
                result.push(Letter::WrongPlace(c));
            } else {
                result.push(Letter::Wrong(c));
            }
        }

        let new_res: Vec<Letter> = result.iter().map(|l| l.clone()).collect();

        self.word_list.push(String::from(word));
        self.tries.push(result);

        Ok(TryResult {
            result: new_res,
            tries: self.get_word_list().len(),
            max_tries: self.get_max_tries(),
        })
    }
}

pub struct TryResult {
    result: Vec<Letter>,
    tries: usize,
    max_tries: usize,
}

impl TryResult {
    pub fn get_game_status(&self) -> GameStatus {
        return if self.result.iter().all(|l| {
            if let Letter::Right(_) = l {
                true
            } else {
                false
            }
        }) {
            GameStatus::Won
        } else if self.tries == self.max_tries {
            GameStatus::Lost
        } else {
            GameStatus::Playing
        };
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Letter {
    Right(char),
    WrongPlace(char),
    Wrong(char),
}

#[derive(Debug)]
pub enum GameStatus {
    Won,
    Playing,
    Lost,
}
