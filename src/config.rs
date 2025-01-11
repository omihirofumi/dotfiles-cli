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
}

impl FileConfig {
    pub fn new(dest: String, src: String) -> Self {
        FileConfig {
            dest: dest,
            src: src,
        }
    }
}
