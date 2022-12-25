use crate::{
    utils,
    utils::{config::Config, Init, NAME},
};

#[tauri::command]
pub fn print(to_print: String) {
    println!("{to_print}");
}
//
// utils
#[tauri::command]
pub fn init() -> Init {
    utils::Init::init()
}

//
// spotify
#[tauri::command]
pub fn spotify_passwd_login(username: &str, passwd: &str) {
    
}