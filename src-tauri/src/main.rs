#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod apis;
mod utils;

use crate::commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![panic_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
