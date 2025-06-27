mod hanged;

use hanged::State;
use rand::prelude::*;
use std::io::{Write, stdin, stdout};

static MAX_ATTEMPTS: &'static u8 = &5;
static MIN_ATTEMPTS: &'static u8 = &0;
static WORDS: &'static [&str] = &[
    "zanahoria",
    "manzana",
    "pera",
    "sandia",
    "huevos",
    "al",
    "backero",
];

fn main() {
    let mut state: State = State::new(*MAX_ATTEMPTS, get_random_word());
    state.set_secret_word();

    println!("Bienvenido al ahorcado, RUSTCADO!");
    play(state);
}

fn play(mut state: State) {
    let complete: bool = state.verify_game_complete();
    let attempts: &u8 = state.get_attempts();

    if complete {
        print!("\n");

        println!("GANASTE! FELICIDADES");
        state.destroy();

        
        let _ = stdout().flush();
        println!("presiona una tecla para continuar");

        stdin()
            .read_line(&mut String::new())
            .unwrap();
        return;
    }

    if attempts == MIN_ATTEMPTS {
        print!("\n");

        println!("Perdiste! vuelve a intentarlo");
        state.destroy();

        let _ = stdout().flush();
        println!("presiona una tecla para continuar");

        stdin()
            .read_line(&mut String::new())
            .unwrap();

        return;
    }

    print!("\n");

    let secret: &str = state.get_secret_word();
    println!("La palabra hasta el momento es: {secret}");

    let guessed_words: String = state.get_guessed_words();
    println!("Adivinaste las siguientes letras: {guessed_words}");

    println!("Te quedan {attempts} intentos.");

    print!("Ingresa una letra: ");

    let mut user_input: String = registry();
    let mut is_valid: bool = validate_input(&user_input);

    while !is_valid {
        print!("Ingresa un valor válido: ");
        user_input = registry();
        is_valid = validate_input(&user_input);
    }

    let letter: char = user_input.chars().next().unwrap();
    state.try_attempt(letter);

    play(state);
}

fn validate_input(target: &str) -> bool {
    let target: &str = target.trim();
    !target.is_empty() && target.len() == 1
}

fn registry() -> String {
    let mut input: String = String::new();
    let _ = stdout().flush();

    stdin().read_line(&mut input).expect("Failed to read line");

    input
}

fn get_random_word() -> &'static str {
    let mut randomizer = rand::rng();
    let target = randomizer.random_range(..WORDS.len());

    WORDS[target]
}
