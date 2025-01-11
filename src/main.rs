mod cli;
mod config;
mod path;

use config::Config;
use path::{copy_path, expand_tilda, home_dir};
use std::{fs, path::Path};

fn main() {
    let config = Config::load(String::from("hoge"));
    config.init();
    copy_path(
        &config.dir(),
        config.dotfiles()[0].src(),
        config.dotfiles()[0].dest(),
    );
}
