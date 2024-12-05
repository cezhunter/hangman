use super::Game;

pub mod ascii;

pub const WELCOME_MESSAGE: &str = "Welcome to Hangman! You know the rules!";
pub const GUESS_MESSAGE: &str = "Please enter your guess.";

pub fn show_game(game: &Game) {
    let word_str = game // might be a cleaner way to do all this
        .public_word
        .iter()
        .map(|&c| c.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let guesses_str = game
        .guesses
        .iter()
        .map(|&c| c.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("Guesses: {}", guesses_str);
    println!("{}", ascii::HANGMAN_ASCII[game.limbs]); //  padding doesn't work here
    println!("{:^19}\n", word_str);
}
