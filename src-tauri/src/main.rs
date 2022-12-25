// there has to be a better way to say we only support linux
#![cfg(target_os = "linux")]

mod apis;
mod utils;
mod commands;

use std::path::PathBuf;

use crate::utils::config::Config;

use crate::commands::*;

fn main() {
    // start tauri once everything is ready
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            print, init
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
