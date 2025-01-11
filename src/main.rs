mod cli;
mod config;

use config::Config;

fn main() {
    let config = Config::load(String::from("hoge"));
    println!("{:?}", config);
}
