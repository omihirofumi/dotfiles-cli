mod cli;
mod config;
mod path;

use config::Config;
use path::{copy_path, expand_tilda};
use std::{fs, path::Path};

fn main() {
    let config = Config::load(String::from("./dotfiles.toml"));
    config.init();
    for (key, value) in config.dotfiles() {
        let dest_dir = Path::new(&config.dir()).join(key);
        fs::create_dir_all(&dest_dir).unwrap();
        copy_path(&expand_tilda(value), &dest_dir);
    }
}
