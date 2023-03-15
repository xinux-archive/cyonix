mod config;
mod linker;
pub mod args;


use std::fs::{create_dir_all, File};
use config::{Config, PATHWAY, STORAGE};
use std::path::{Path};

pub const CONFIG: &str = ".config";

#[derive(Debug)]
pub struct Cyonix<'a> {
    pub config: Config<'a>,
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
    pub fn move_file(&self, file: &str) -> std::io::Result<()>{
        let current_path: &Path = Path::new(file);
        let storage_path =  format!("{}{}{}", PATHWAY, STORAGE, file);

        create_dir_all(PATHWAY.to_string() + STORAGE)?;

        if current_path.exists() {
            std::fs::copy(file, storage_path)?;
            std::fs::remove_file(file)?;
        }

        Ok(())
    }

    pub fn add(&self, file: &str) -> std::io::Result<()> {
        self.move_file(file)?;
        File::create(file)?;
        Ok(())
    }



}
