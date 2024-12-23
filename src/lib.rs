use rand::Rng;
use std::fs;
use std::io;

mod display;

const MAX_LIMBS: usize = 6;

enum GuessResult {
    // might need to associate with struct
    AlreadyGuessed(char),
    SuccessfulGuess(usize),
    FailedGuess,
    OutOfTurns,
    GameWon,
}

struct Game<'a> {
    secret_word: &'a String,
    guesses: Vec<char>,
    public_word: Vec<char>,
    limbs: usize,
}

impl<'a> Game<'a> {
    fn register_guess(&mut self, guess: char) -> GuessResult {
        if self.guesses.contains(&guess) || self.public_word.contains(&guess) {
            return GuessResult::AlreadyGuessed(guess);
        }
        let found_indices: Vec<_> = self
            .secret_word
            .char_indices()
            .filter(|(_, c)| c == &guess)
            .map(|(i, c)| self.public_word[i] = c)
            .collect();
        let num_matches = found_indices.len();
        if self.public_word.iter().collect::<String>() == *self.secret_word {
            return GuessResult::GameWon;
        } else if num_matches > 0 {
            return GuessResult::SuccessfulGuess(num_matches);
        }
        self.limbs += 1;
        self.guesses.push(guess);
        if self.limbs >= MAX_LIMBS {
            GuessResult::OutOfTurns
        } else {
            GuessResult::FailedGuess
        }
    }
}

fn read_words(file_path: &str) -> Vec<String> {
    let words = fs::read_to_string(file_path)
        .unwrap()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    words
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
    };
    println!("{}", display::WELCOME_MESSAGE);
    display::show_game(&game);
    loop {
        println!("{}", display::GUESS_MESSAGE);
        let mut guess = String::new(); // this code is boilerplate from the docs, could clean up
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read guess");
        let guess: char = match guess.trim().parse() {
            Ok(g) => g,
            Err(_) => continue,
        };
        if !guess.is_alphabetic() {
            // really should be part of a parsing func
            continue;
        }
        let res: GuessResult = game.register_guess(guess);
        display::show_game(&game);
        match res {
            GuessResult::AlreadyGuessed(c) => println!("already guessed {}", c),
            GuessResult::SuccessfulGuess(n) => println!("found {} matching", n),
            GuessResult::FailedGuess => println!("nope!"),
            GuessResult::GameWon => {
                println!("Congrats! You won!");
                break;
            }
            GuessResult::OutOfTurns => {
                println!("game over. word was {}", game.secret_word);
                break;
            }
        }
    }
}
