mod cli;
mod config;
mod path;

use config::Config;
use path::{copy_path, expand_tilda, home_dir, resolve_path};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn main() {
    let config = Config::load(String::from("hoge"));
    config.init();
    config.dotfiles().iter().for_each(|dotfile| {
        let dest_dir = Path::new(&config.dir()).join(dotfile.name());
        fs::create_dir_all(&dest_dir).unwrap();
        copy_path(&dest_dir.to_string_lossy(), dotfile.src(), dotfile.dest());
    });
}
