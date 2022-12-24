use crate::utils::{NAME, config::Config};

// debug stuff
#[derive(serde::Serialize)]
pub struct custom_struct {
    custom_item: i32
}

#[tauri::command]
pub fn custom_cmd() -> custom_struct {
    custom_struct { custom_item: 79 }
}

#[tauri::command]
pub fn print_test(to_print: String) {
    println!("{to_print}");
}
//
// utils
#[tauri::command]
pub fn init() -> (xdg::BaseDirectories, Config) {
    let xdg_dirs =
        xdg::BaseDirectories::with_prefix(NAME).expect("Failed to create xdg directoreis");
    
    let cfg = Config::load(&xdg_dirs);

    return (xdg_dirs, cfg);
}