use crate::{utils, utils::{Init, NAME, config::Config}};

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
pub fn init() -> Init {
    utils::init()
}