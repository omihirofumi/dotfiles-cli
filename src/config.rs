#[derive(Debug)]
pub struct Config {
    dir: String,
    dotfiles: Vec<FileConfig>,
}

#[derive(Debug)]
pub struct FileConfig {
    dest: String,
    src: String,
}

impl Config {
    pub fn new(dir: String, dotfiles: Vec<FileConfig>) -> Self {
        Config {
            dir: dir,
            dotfiles: dotfiles,
        }
    }
    pub fn load(filename: String) -> Self {
        let _ = filename;
        Config {
            dir: "~/temp/dotfiles".to_string(),
            dotfiles: vec![FileConfig {
                dest: "wezterm".to_string(),
                src: "~/.wezterm.lua".to_string(),
            }],
        }
    }
    pub fn dir(&self) -> &str {
        &self.dir
    }
    pub fn dotfiles(&self) -> &Vec<FileConfig> {
        &self.dotfiles
    }
}

impl FileConfig {
    pub fn new(dest: String, src: String) -> Self {
        FileConfig {
            dest: dest,
            src: src,
        }
    }
}
