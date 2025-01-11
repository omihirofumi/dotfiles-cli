use std::path::PathBuf;

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
