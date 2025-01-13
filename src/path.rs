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
    let filename = src.file_name().ok_or("cannot get filename").unwrap();
    let dest_path = dest.join(filename);

    println!(
        "copying {} to {}...",
        src.to_string_lossy(),
        dest_path.to_string_lossy()
    );
    if src.is_dir() {
        copy_dir(&src, &dest_path).unwrap();
    } else {
        fs::copy(src, dest_path).unwrap();
    }
}

pub fn resolve_path(dir: &str, name: &str) -> PathBuf {
    Path::new(dir).join(name)
}
