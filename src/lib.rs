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
    pub fn find_homedir(&self) -> PathBuf {
        home_dir().unwrap()
    }

    pub fn find_config(&self) -> PathBuf {
        self.find_homedir().join(CONFIG)
    }

    pub fn create_dirs(&self) -> std::io::Result<()>{
        let path = PathBuf::from(PATHWAY.to_string() + STORAGE);

        if !path.exists() {
            create_dir_all(self.find_homedir().join(PATHWAY.to_string() + STORAGE))?;
        }

        let file_list = self.find_homedir().join(PATHWAY).join("file.list");
        File::create(file_list)?;

        Ok(())
    }

    // move files to .cyonix
    pub fn move_file(&self, file: &str) -> std::io::Result<()>{
        self.create_dirs()?;
        let config_path = self.find_homedir().join(CONFIG).join(file);
        let storage_path =  format!("{}{}{}", PATHWAY, STORAGE, file);


        if config_path.exists() {
            std::fs::copy(&config_path, storage_path)?;
            std::fs::remove_file(&config_path)?;
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
