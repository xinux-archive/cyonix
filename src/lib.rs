mod config;
mod linker;
pub mod args;


use std::fs::{create_dir_all, File, Permissions, remove_file};
use config::{Config, PATHWAY, STORAGE};
use std::path::{Path, PathBuf};
use dirs::home_dir;

pub const CONFIG: &str = ".config";

#[derive(Debug)]
pub struct Cyonix<'a> {
    pub config: Config<'a>,
}

impl<'a> Default for Cyonix<'a> {
    fn default() -> Self {
        let location = home_dir().unwrap();

        Cyonix {
            config: Config::new(&location),
        }
    }
}

impl<'a> Cyonix<'a> {
    pub fn create_dirs(&self, file: &str) -> std::io::Result<()>{
        let home = home_dir().unwrap();
        let path = PathBuf::from(PATHWAY.to_string() + STORAGE);

        if !path.exists() {
            create_dir_all(home.join(PATHWAY.to_string() + STORAGE))?;
        }

        let file_path = home.join(PATHWAY);
        let file_list = file_path.join(file);

        File::create(file_list)?;

        Ok(())
    }

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
        let in_storage = format!("{}{}{}", PATHWAY, STORAGE, file);
        File::create(in_storage)?;
        Ok(())
    }
}
