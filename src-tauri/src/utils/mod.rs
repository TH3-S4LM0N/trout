pub mod config;

use config::Config;
use serde::{Serialize, Deserialize};

pub const NAME: &str = env!("CARGO_CRATE_NAME");
pub const CONFIG_NAME: &str = "trout.json";

// https://github.com/whitequark/rust-xdg/pull/49
// just have to wait until this is merged ig
#[derive(Serialize, Deserialize)]
pub struct Init {
    pub is_firstboot: bool,
    pub cfg: config::Config,
    pub xdg_dirs: xdg::BaseDirectories,
}

pub fn init() -> Init {
    let xdg_dirs =
        xdg::BaseDirectories::with_prefix(NAME).expect("Failed to create basedirs struct");
    let (cfg, is_firstboot) = Config::load(&xdg_dirs);
    return Init {
        is_firstboot: is_firstboot,
        cfg: cfg,
        xdg_dirs: xdg_dirs
    };
}
