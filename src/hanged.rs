use std::fmt::Write;

pub struct State {
    attempts: u8,
    max_attempts: u8,
    word: String,
    secret: String,
    guessed: String,
}

impl State {
    pub fn new(attempts: u8, word: &str) -> State {
        State {
            attempts,
            max_attempts: attempts,
            word: word.to_lowercase(),
            secret: String::new(),
            guessed: String::new(),
        }
    }

    pub fn get_attempts(&self) -> &u8 {
        &self.attempts
    }

    pub fn get_secret_word(&self) -> &str {
        &self.secret
    }

    pub fn set_secret_word(&mut self) {
        for _ in 0..self.word.len() {
            self.secret.push_str("_ ");
        }
    }

    pub fn try_attempt(&mut self, letter: char) {
        let is_owned: bool = self.verify_already_owned(letter);
        if is_owned {
            return;
        }

        let word_exist: bool = self.verify_word_exist(letter);

        match word_exist {
            true => self.registry_success_attempt(letter),
            false => self.registry_fail_attempt(),
        }
    }

    fn verify_word_exist(&self, letter: char) -> bool {
        let mut exist: bool = false;

        for char in self.word.chars() {
            exist = char.to_string().trim() == letter.to_string().trim();
            if exist {
                break;
            }
        }

        exist
    }

    fn verify_already_owned(&self, letter: char) -> bool {
        let mut owned: bool = false;

        for char in self.guessed.chars() {
            owned = char.to_string().trim() == letter.to_string().trim();
            if owned {
                break;
            }
        }

        owned
    }

    pub fn destroy(self) {
        print!("\n");

        println!("Resumen de la partida");
        println!("Letras adivinadas: {}", self.guessed.len());
        println!("Intentos fallidos: {}", self.max_attempts - self.attempts);
        println!("Palabra: {}", self.word.to_uppercase())
    }

    pub fn verify_game_complete(&self) -> bool {
        let filtered: Vec<&str> = self.secret.split(" ").filter(|&c| c != "_").collect();
        let word: String = filtered.join("");

        self.word == word.to_lowercase()
    }

    fn registry_success_attempt(&mut self, letter: char) {
        let _ = self.guessed.write_char(letter);

        let mut letters_to_replace: Vec<usize> = vec![];

        for char in self.word.chars().enumerate() {
            let index: usize = char.0.try_into().unwrap();
            let char: char = char.1;

            if char.to_string().trim() == letter.to_string().trim() {
                letters_to_replace.push(index);
            }
        }

        let mut parts: Vec<String> = self
            .secret
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        for &index in &letters_to_replace {
            parts[index] = letter.to_string();
        }

        let secret: String = parts.join(" ").to_uppercase();
        self.secret = secret;
    }

    fn registry_fail_attempt(&mut self) {
        self.attempts -= 1;
    }

    pub fn get_guessed_words(&self) -> String {
        let upper_case: String = self.guessed.to_uppercase();
        upper_case
    }
}
