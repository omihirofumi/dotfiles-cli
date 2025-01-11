mod config;

use config::{Config, FileConfig};

fn main() {
    let config = Config::new(
        "dir_hoge".to_string(),
        vec![FileConfig::new(
            "dest_hoge".to_string(),
            "src_hoge".to_string(),
        )],
    );
    println!("{:?}", config);
}
