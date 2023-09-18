// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn modular_shift(value: i64, module: i64) -> u64 {
    // Always positive
    ((module + (value % module)) % module) as u64
}

#[tauri::command]
fn cesar_solve(alphabet: &str, message: &str, shift: i64) -> String {
    // We garant on frontend that values exactly exists
    message.chars().into_iter()
        .map(|c| {
            let pos: i64 = alphabet.chars()
                .position(|a| a == c)
                .unwrap()
                .try_into()
                .unwrap();

            let complete_shift: usize = modular_shift(pos + shift, alphabet.chars().count().try_into().unwrap())
                .try_into()
                .unwrap();

            alphabet.chars().nth(complete_shift).unwrap()
        })
        .collect()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![cesar_solve])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
