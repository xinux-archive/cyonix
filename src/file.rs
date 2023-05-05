use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use crate::config::{CONFIG, FILE, PATHWAY, STORAGE};
use crate::error::CyonixError;

#[derive(Debug)]
pub struct Files {
    home: PathBuf
}

impl Files {
    pub fn new(home: PathBuf) -> Files {
        Files{
            home
        }
    }

    pub fn find_storage(&self) -> PathBuf {
        self.home.join(PATHWAY.to_string() + STORAGE)
    }
    
    pub fn find_filelist(&self) -> PathBuf {
        self.home.join(PATHWAY).join(FILE)
    }

    pub fn write_to_list(&self, file: &str) -> Result<(), CyonixError>{
        let file_list = self.find_filelist();
        let file_location = format!(" ~/{}/{}\n", CONFIG, file);

        if !file_list.exists(){
           File::create(&file_list)?;
        }

        let opened_file = OpenOptions::new()
            .append(true)
            .open(file_list)?;

        let mut writer = BufWriter::new(opened_file);
        writer.write_all(file.as_bytes())?;
        writer.write_all(file_location.as_bytes())?;
        writer.flush()?;

        Ok(())
    }

    pub fn create_dirs(&self) -> Result<(), CyonixError>{
        let path = PathBuf::from(PATHWAY.to_string() + STORAGE);
        if !path.exists() {
            create_dir_all(self.find_storage())?;
        }
        Ok(())
    }

    // move files to .cyonix
    pub fn move_file(&self, file: &str) -> Result<(), CyonixError>{
        self.create_dirs()?;
        self.write_to_list(file)?;

        let config_path = self.home.join(CONFIG).join(file);
        let storage_path =  self.find_storage().join(file);

        if config_path.exists() {
            std::fs::copy(&config_path, storage_path)?;
            std::fs::remove_file(&config_path)?;
        }
        Ok(())
    }

    pub fn delete(&self, file: &str) -> Result<(), CyonixError> {
        self.move_file(file)?;
        let storage_path = self.find_storage().join(file);
        if storage_path.exists() {
            std::fs::remove_file(storage_path)?;
        }
        Ok(())
    }

}