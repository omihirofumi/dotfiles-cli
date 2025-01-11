mod cli;
mod config;

use config::{Config, FileConfig};

fn main() {
    let config = Config::load(String::from("hoge"));
    println!("{:?}", config);
}
