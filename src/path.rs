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

pub fn copy_path(dir: &str, src: &str, dest: &str) {
    let dest_path = expand_tilda(dir).join(dest);
    let src_path = expand_tilda(src);
    println!(
        "copying {} to {}...",
        src_path.to_string_lossy(),
        dest_path.to_string_lossy()
    );
    if src_path.is_dir() {
        copy_dir(&src_path, &dest_path).unwrap();
    } else {
        fs::copy(src_path, dest_path).unwrap();
    }
}

pub fn resolve_path(dir: &str, name: &str) -> PathBuf {
    Path::new(dir).join(name)
}
