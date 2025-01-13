use std::{fs, path::PathBuf};

use crate::path::expand_tilda;

#[derive(Debug)]
pub struct Config {
    dir: String,
    dotfiles: Vec<FileConfig>,
}

#[derive(Debug)]
pub struct FileConfig {
    name: String,
    dest: String,
    src: String,
}

impl Config {
    pub fn load(filename: String) -> Self {
        let _ = filename;
        Config {
            dir: "~/temp/dotfiles".to_string(),
            dotfiles: vec![FileConfig {
                name: "wezterm".to_string(),
                dest: "wezterm.lua".to_string(),
                src: "~/.wezterm.lua".to_string(),
            }],
        }
    }
    pub fn init(&self) {
        fs::create_dir_all(self.dir()).unwrap();
    }
    pub fn dir(&self) -> String {
        expand_tilda(&self.dir)
            .to_str()
            .ok_or("cannot parse dir string")
            .unwrap()
            .to_string()
    }
    pub fn dotfiles(&self) -> &Vec<FileConfig> {
        &self.dotfiles
    }
}

impl FileConfig {
    pub fn new(name: String, dest: String, src: String) -> Self {
        FileConfig {
            name: name,
            dest: dest,
            src: src,
        }
    }
    pub fn src(&self) -> &str {
        &self.src
    }
    pub fn dest(&self) -> &str {
        &self.dest
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
