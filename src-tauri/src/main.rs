// there has to be a better way to say we only support linux
#![cfg(target_os = "linux")]

mod apis;
mod utils;
mod commands;

use std::path::PathBuf;

use crate::utils::config::Config;

const NAME: &str = env!("CARGO_CRATE_NAME");
const CONFIG_NAME: &str = "trout.json";

use crate::commands::*;

fn main() {
    // create a handful of important values that
    // will be used all throught the rust part
    let xdg_dirs =
        xdg::BaseDirectories::with_prefix(NAME).expect("Failed to create xdg directoreis");
    let cfg = Config::load(&xdg_dirs);


    // start tauri once everything is ready
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            print_test, custom_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
