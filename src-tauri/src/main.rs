// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Mutex, vec};

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
enum LetterStatus {
    Correct,
    Mistake,
    Suggestion,
}

#[derive(Debug, Clone, serde::Serialize)]
struct Letter {
    character: char,
    status: LetterStatus,
}

impl Letter {
    pub fn new(character: char, status: LetterStatus) -> Self {
        Self { character, status }
    }

    pub fn suggestion(character: char) -> Self {
        Self::new(character, LetterStatus::Suggestion)
    }

    pub fn mistake(character: char) -> Self {
        Self::new(character, LetterStatus::Mistake)
    }
}

#[derive(Default, Debug, serde::Serialize)]
struct Typeru {
    text: Vec<String>,
    cursor_word: Mutex<usize>,
    cursor_index: Mutex<usize>,
    input: Mutex<Vec<Vec<Letter>>>,
}

impl Typeru {
    pub fn new(text: &str) -> Self {
        let text: Vec<String> = text.split(' ').map(|str| str.to_string()).collect();
        let mut input: Vec<Vec<Letter>> = vec![];

        for word in &text {
            let mut input_word: Vec<Letter> = vec![];
            for character in word.chars() {
                input_word.push(Letter::suggestion(character))
            }
            input.push(input_word)
        }

        Self {
            text,
            input: Mutex::from(input),
            ..Default::default()
        }
    }

    pub fn insert(&self, character: char) {
        let mut input = self.input.lock().unwrap();
        let input_len = input.len();
        let cursor_word = *self.cursor_word.lock().unwrap();
        let cursor_index = *self.cursor_index.lock().unwrap();

        let input_word = &mut input[cursor_word];
        let text_word = &self.text[cursor_word];

        if character == ' ' {
            if cursor_word < input_len - 1 {
                let input_word_len = input_word.len();
                for letter in input_word.get_mut(cursor_index..input_word_len).unwrap() {
                    letter.status = LetterStatus::Mistake;
                }
                *self.cursor_index.lock().unwrap() = 0;
                *self.cursor_word.lock().unwrap() += 1;
            }
            return;
        }

        if cursor_index >= text_word.len() {
            if cursor_word >= self.text.len() - 1 {
                return;
            }

            input_word.push(Letter::mistake(character));
            *self.cursor_index.lock().unwrap() += 1;
            return;
        }

        let text_character = text_word.chars().nth(cursor_index).unwrap();
        let input_letter = &mut input_word[cursor_index];
        if character == text_character {
            input_letter.status = LetterStatus::Correct;
        } else {
            input_letter.status = LetterStatus::Mistake;
        }
        *self.cursor_index.lock().unwrap() += 1;
    }

    pub fn backspace(&self) {
        let mut input = self.input.lock().unwrap();
        let cursor_word = *self.cursor_word.lock().unwrap();
        let cursor_index = *self.cursor_index.lock().unwrap();

        if cursor_index == 0 {
            if cursor_word == 0 {
                return;
            }

            let new_word = &mut input[cursor_word - 1];
            *self.cursor_index.lock().unwrap() = new_word.len();
            *self.cursor_word.lock().unwrap() -= 1;
            return;
        }

        let input_word = &mut input[cursor_word];
        let text_word = &self.text[cursor_word];
        if cursor_index > text_word.len() {
            input_word.pop();
        } else {
            let previous_character = &mut input_word[cursor_index - 1];
            previous_character.status = LetterStatus::Suggestion;
        }
        *self.cursor_index.lock().unwrap() -= 1;
    }

    pub fn clone_input(&self) -> Vec<Vec<Letter>> {
        self.input.lock().unwrap().clone()
    }
}

#[tauri::command]
fn get_cursor(state: tauri::State<Typeru>) -> usize {
    *state.cursor_word.lock().unwrap()
}

#[tauri::command]
fn get_input(state: tauri::State<Typeru>) -> Vec<Vec<Letter>> {
    state.clone_input()
}

#[tauri::command]
fn insert(character: char, state: tauri::State<Typeru>) -> Vec<Vec<Letter>> {
    state.insert(character);
    state.clone_input()
}

#[tauri::command]
fn backspace(state: tauri::State<Typeru>) -> Vec<Vec<Letter>> {
    state.backspace();
    state.clone_input()
}

fn main() {
    let typeru = Typeru::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur efficitur ante fringilla, sodales tortor eu, imperdiet nibh. Quisque eu nulla ullamcorper, vestibulum urna ac, tempor elit. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Quisque id ante eget nisl pharetra rutrum porta ut nisi. Sed blandit pulvinar tempus. Maecenas mollis lobortis ipsum. Curabitur consectetur blandit porttitor. Sed vel tellus ligula. Pellentesque in augue ut enim accumsan bibendum id id metus. Ut metus ipsum, congue in mauris a, imperdiet sagittis nibh. Fusce consectetur diam ac quam fermentum, vel iaculis eros porta. Integer eu egestas magna. Morbi ante libero, varius quis posuere nec, dignissim vel felis. Curabitur tincidunt auctor justo, vitae mattis nulla dictum at.");

    tauri::Builder::default()
        .manage(typeru)
        .invoke_handler(tauri::generate_handler![
            get_cursor, get_input, insert, backspace
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
