mod cli;
mod config;
mod path;

use config::Config;
use path::{expand_tilda, home_dir};
use std::{fs, path::Path};

fn main() {
    let config = Config::load(String::from("hoge"));
    println!("{:?}", config);
    let dir = config.dir();
    println!("{}", expand_tilda(dir).to_string_lossy());
}
