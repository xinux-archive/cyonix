mod config;
mod linker;

use config::Config;
use std::path::{Path, PathBuf};
use config::PATHWAY;

#[derive(Debug)]
pub struct Cyonix {
    config: Config,
}

impl Default for Cyonix {
    fn default() -> Self {
        let location = home::home_dir().unwrap();

        Cyonix {
            config: Config::new(&location),
        }
    }
}

impl Cyonix {
    // move files to .cyonix
    pub fn move_file(&self, file: &str) {
        let path: &Path = Path::new(file);

        if path.exists() {
            std::fs::copy(file, format!("{}{}", PATHWAY, file)).expect("Failed to copy file");
            std::fs::remove_file(file).expect("Failed to remove file");
        }
    }

    pub fn add(&self, file: &str) {
        self.move_file(file);
    }
}
