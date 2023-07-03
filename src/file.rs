use std::fs;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{BufWriter, ErrorKind, Write};
use std::path::{Path, PathBuf};
use crate::config::{CONFIG, FILE, PATHWAY, STORAGE};
use crate::error::CyonixError;
use walkdir::WalkDir;

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

    pub fn validate_file(&self, file: &str) -> Result<(), CyonixError> {
        if !Path::new(file).exists(){
            return Err(CyonixError::CustomError(ErrorKind::NotFound))
        }
        if !fs::metadata(file)?.is_file(){
            return Err(CyonixError::SpecificError(format!("{} is not a file", file)))
        }
        Ok(())
    }

    pub fn copy_recursively<P: AsRef<Path>>(&self, src: P, dest: P) -> Result<(), CyonixError> {
        for entry in WalkDir::new(src.as_ref()){
            let entry = entry.unwrap();
            let path = entry.path();
            let mut new_path = dest.as_ref().to_path_buf();
            new_path.push(path.strip_prefix(src.as_ref()).unwrap());
            if path.is_dir() {
                fs::create_dir_all(new_path)?;
            } else {
                fs::copy(path, new_path)?;
            }
        }
        Ok(())
    }

    pub fn write_to_list(&self, file: &str) -> Result<(), CyonixError>{
        self.validate_file(file)?;
        let file_list = self.find_filelist();

        let collection: Vec<&str> = file.split('/')
                                        .filter(|f| !f.is_empty())
                                        .collect();

        let file_name = collection.last().unwrap_or(&"Oops, failed to write the file :(");

        if !file_list.exists(){
           File::create(&file_list)?;
        }

        let opened_file = OpenOptions::new()
            .append(true)
            .open(file_list)?;

        let mut writer = BufWriter::new(opened_file);
        writer.write_all(file_name.as_bytes())?;
        writer.write_all(b" ")?;
        writer.write_all(file.as_bytes())?;
        writer.write_all(b"\n")?;
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

        let storage_path =  self.find_storage().join(file);
        let storage_string = storage_path
            .to_str()
            .unwrap_or("Oops, failed to resolve path into string :(");

        self.copy_recursively(file, storage_string)?;

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