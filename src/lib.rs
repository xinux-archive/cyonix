mod config;
mod linker;
pub mod args;


use config::{Config, PATHWAY};
use std::path::{Path};

#[derive(Debug)]
pub struct Cyonix<'a> {
    config: Config<'a>,
}

impl<'a> Default for Cyonix<'a> {
    fn default() -> Self {
        let location = home::home_dir().unwrap();

        Cyonix {
            config: Config::new(&location),
        }
    }
}

impl<'a> Cyonix<'a> {
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
