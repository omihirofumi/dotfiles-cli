use std::{
    fs,
    path::{Path, PathBuf},
};

use dircpy::copy_dir;

pub fn home_dir() -> PathBuf {
    dirs::home_dir()
        .ok_or("cannot parse home directory")
        .unwrap()
}

pub fn expand_tilda(path: &str) -> PathBuf {
    if path.starts_with("~") {
        let home = home_dir();
        home.join(&path[2..])
    } else {
        PathBuf::from(path)
    }
}

pub fn copy_path(src: &Path, dest: &Path) {
    println!(
        "copying {} to {}...",
        src.to_string_lossy(),
        dest.to_string_lossy()
    );
    if src.is_dir() {
        copy_dir(src, dest).unwrap();
    } else {
        let filename = src.file_name().ok_or("cannot get filename").unwrap();
        let dest_path = dest.join(filename);
        fs::create_dir_all(dest).unwrap();
        fs::copy(src, dest_path).unwrap();
    }
}
