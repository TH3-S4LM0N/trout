use crate::utils::CONFIG_NAME;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, write, File},
    path::PathBuf, io::Write,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub spotify_username: String,
    pub spotify_password: String,
}

impl Config {
    // doesnt actually work yet as i have no encryption method
    // will figure out b4 1.0
    pub fn hash_passwd(&mut self) {}

    pub fn new(path: &PathBuf) {
        let empty = Config {
            spotify_username: String::new(),
            spotify_password: String::new(),
        };

        let mut file = File::create(&path).expect("Failed to create config file");
        file.write_all(&serde_json::to_string(&empty).expect("Failed to create empty JSON Config").as_bytes()).expect("Failed to write config file");
    }

    pub fn default() -> Config {
        Config {
            spotify_username: String::new(),
            spotify_password: String::new()
        }
    }

    pub fn write_to(&self, path: &PathBuf) {
        let json = serde_json::to_string(self).expect("Failed to create JSON config object");
        write(&path, &json).expect("Failed to write JSON to config file");
    }

    pub fn load(xdg_dirs: &xdg::BaseDirectories) -> (Config, bool) {
        // loop to check if config file is valid
        // if it isnt err or smthn, create a new one
        // and try again
        let cfg_path = xdg_dirs.find_config_file(CONFIG_NAME);
        let mut cfg = Config::default();
        let mut is_firstboot = false;
        loop {
            // does the xdg cfg exist
            let is = *&cfg_path.is_none();
            // if it doesnt exist we get the new xdg path
            // and create a default config at said path
            if is {
                let new_cfg_path = xdg_dirs.get_config_file(CONFIG_NAME);
                println!("{:?}", &new_cfg_path);
                Config::new(&new_cfg_path);
                is_firstboot = true;
            // if it does exist then we read it into the struct
            } else {
                cfg = serde_json::from_str(
                    &fs::read_to_string(&cfg_path.unwrap())
                        .expect("Failed to read config file into string"),
                )
                .expect("Failed to read config into json");
                break;
            }
        }

        return (cfg, is_firstboot);
    }
}
