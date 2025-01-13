use std::{collections::HashMap, fs};

use serde::Deserialize;

use crate::path::expand_tilda;

#[derive(Debug, Deserialize)]
pub struct Config {
    general: General,
    dotfiles: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct General {
    dir: String,
}

impl Config {
    pub fn load(filename: String) -> Self {
        let config_toml = fs::read_to_string(filename).unwrap();
        toml::from_str(&config_toml).unwrap()
    }
    pub fn init(&self) {
        fs::create_dir_all(self.dir()).unwrap();
    }
    pub fn dir(&self) -> String {
        expand_tilda(&self.general.dir)
            .to_str()
            .ok_or("cannot parse dir string")
            .unwrap()
            .to_string()
    }
    pub fn dotfiles(&self) -> &HashMap<String, String> {
        &self.dotfiles
    }
}
