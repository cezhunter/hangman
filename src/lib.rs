use rand::Rng;
// use std::fmt;
use std::fs;
use std::io;

mod ascii;

enum GameStatus {
    AlreadyGuessed(char),
    SuccessfulGuess(usize),
    FailedGuess,
    OutOfTurns,
    GameWon,
    Pending,
}

struct Game<'a> {
    secret_word: &'a String,
    guesses: Vec<char>,
    public_word: Vec<char>,
    limbs: usize,
    status: GameStatus,
}

impl<'a> Game<'a> {
    fn register_guess(&mut self, guess: char) {
        if self.guesses.contains(&guess) || self.public_word.contains(&guess) {
            self.status = GameStatus::AlreadyGuessed(guess);
            return;
        }
        let found_indices: Vec<_> = self
            .secret_word
            .char_indices()
            .filter(|(_, c)| c == &guess)
            .map(|(i, c)| self.public_word[i] = c)
            .collect();
        let num_matches = found_indices.len();
        if self.public_word.iter().collect::<String>() == *self.secret_word {
            self.status = GameStatus::GameWon;
        } else if num_matches > 0 {
            self.status = GameStatus::SuccessfulGuess(num_matches);
        } else {
            self.limbs += 1;
            self.guesses.push(guess);
            if self.limbs >= 6 {
                self.status = GameStatus::OutOfTurns;
            } else {
                self.status = GameStatus::FailedGuess;
            }
        }
    }
}

fn vec_to_string(vec: &Vec<char>) -> String {
    vec.iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn show_game(game: &Game) {
    let message = match game.status {
        GameStatus::AlreadyGuessed(c) => format!("already guessed {}", c),
        GameStatus::SuccessfulGuess(n) => format!("found {} matching", n),
        GameStatus::FailedGuess => String::from("Sorry, not correct"),
        GameStatus::GameWon => String::from("Congrats! You won!"),
        GameStatus::OutOfTurns => format!("GAME OVER. The word was {}", game.secret_word),
        GameStatus::Pending => String::new(),
    };
    let guesses_string = vec_to_string(&game.guesses);
    let word_string = vec_to_string(&game.public_word);
    println!(
        "Guesses: {}\n{}\n{:^19}\n{}\n",
        guesses_string,
        ascii::HANGMAN_ASCII[game.limbs],
        word_string,
        message
    );
}

fn read_words(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn get_guess() -> Option<char> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read guess");
    match guess.trim().parse::<char>() {
        Ok(g) if g.is_alphabetic() => Some(g),
        _ => None,
    }
}

pub fn start_game() {
    let words = read_words("src/words.txt");
    let rand_num = rand::thread_rng().gen_range(0..words.len());
    let secret_word = &words[rand_num];
    let mut game = Game {
        secret_word,
        guesses: Vec::new(),
        public_word: vec!['_'; secret_word.len()],
        limbs: 0,
        status: GameStatus::Pending,
    };
    println!("{}", "Welcome to Hangman! You know the rules!");
    show_game(&game);
    loop {
        println!("{}", "Please enter your guess.");
        let guess = match get_guess() {
            Some(g) => g,
            None => continue,
        };
        game.register_guess(guess);
        show_game(&game);
        match &game.status {
            GameStatus::GameWon | GameStatus::OutOfTurns => break,
            _ => {}
        }
    }
}
