// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

#[derive(Default, Debug, serde::Serialize)]
struct Typeru {
    text: String,
    input: Mutex<Vec<String>>,
}

impl Typeru {
    pub fn new() -> Self {
        Self {
            text: String::from("Lorem ipsum dolor sit amet"),
            ..Default::default()
        }
    }

    pub fn insert(&self, character: char) {
        let mut input = self.input.lock().unwrap();
        if character == ' ' {
            input.push(String::from(""));
        } else if let Some(word) = input.last_mut() {
            word.push(character);
        } else {
            input.push(character.to_string());
        }
    }

    pub fn backspace(&self) {
        let mut input = self.input.lock().unwrap();
        if let Some(word) = input.last_mut() {
            if word.is_empty() {
                input.pop();
            } else {
                word.pop();
            }
        }
    }

    pub fn clone_input(&self) -> Vec<String> {
        self.input.lock().unwrap().clone()
    }
}

#[tauri::command]
fn get_text(state: tauri::State<Typeru>) -> String {
    state.text.clone()
}

#[tauri::command]
fn insert(character: char, state: tauri::State<Typeru>) -> Vec<String> {
    state.insert(character);
    state.clone_input()
}

#[tauri::command]
fn backspace(state: tauri::State<Typeru>) -> Vec<String> {
    state.backspace();
    state.clone_input()
}

fn main() {
    tauri::Builder::default()
        .manage(Typeru::new())
        .invoke_handler(tauri::generate_handler![get_text, insert, backspace])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
