// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cipher;
mod math;
mod hack;
mod alphabets;

use alphabets::{RussianAlphabet, Alphabet};
use cipher::cesar::Cesar;
use hack::frequency::FrequencyAnalysis;

#[tauri::command]
fn cesar_solve(alphabet: &str, message: &str, shift: i64, ignore: bool) -> String {
    let cesar = Cesar::new(message, alphabet);

    cesar.encrypt(shift, ignore)
}

#[tauri::command]
fn frequency_analysis(message: &str) -> String {
    let binding = RussianAlphabet::get_alphabet();
    let cesar = Cesar::new(message, binding.as_str());
    let freq_analysis = FrequencyAnalysis::new(&cesar);

    freq_analysis.decrypt()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![cesar_solve, frequency_analysis])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
