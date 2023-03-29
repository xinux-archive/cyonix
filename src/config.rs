use std::fmt::Debug;
use std::path::{Path, PathBuf};
use dirs::home_dir;
use crate::error::CyonixError;


#[cfg(target_os = "macos")]
pub const PATHWAY: &str = ".cyonix";

#[cfg(target_os = "windows")]
pub const PATHWAY: &str = "AppData/Roaming/cyonix";

#[cfg(target_os = "linux")]
use std::os::unix::fs;
pub const PATHWAY: &str = ".cyonix";
pub const STORAGE: &str = "/storage";
pub const FILE: &str = "/file.list";

/// Find PathBuf of config directory
pub fn config_directory() -> PathBuf {
     base_directory().join(".config")
}

/// Find PathBuf of base home directory
pub fn base_directory() -> PathBuf {
    home_dir().expect("Could not find home directory :(")
}

#[derive(Debug, Clone)]
pub struct Config<'a> {
    /// Location of config folder
    home: String,
    
    /// List of files to be linked
    files: Vec<(&'a str, &'a str)>,
}

impl <'a> Config<'a> {
    /// Load instance of Config
    /// Temporarily left default configs, will be changed soon!
    pub fn new(path: &Path) -> Config<'a> {
      Config {
        home: path.to_str().unwrap().to_string(),
        files: Vec::new(),
      }
    }
  
    /// Parse the file.list file
    /// where name and location of files are stored
    ///
    /// .zshrc   ~/.zshrc
    /// .bashrc  ~/.bashrc
    ///       ^     ^
    ///       |     |
    /// Vec<(&str,&str)>
    /// < file >< whitespace >< location >< new line >
    pub fn parse(&mut self, file: &'a str) {
        let lines = file.lines();
        
        for line in lines {
            let mut words = line.split_whitespace();

            let name = words.next().unwrap();
            let location = words.next().unwrap();
            
            self.files.push((name, location));
        }
    }
  
     pub fn read(&self, file: &'a str) -> Result<&'a str, CyonixError> {
         let conf_file = config_directory().join(file);
         if !conf_file.exists(){
             return Err(CyonixError::CustomError(String::from("The file doesn't exist :(")))
         }

         std::fs::read_to_string(conf_file).expect("Failed to read :(");

         Ok(file)
    }

    pub fn create_symlinks(&self) -> Result<(), CyonixError> {
        for (name, location) in &self.files {
            let source = Path::new(&self.home).join(name);
            let target = Path::new(location);

            if target.exists() {
                return Err(CyonixError::CustomError(format!("Target file already exists: {}", location)));
            }

            fs::symlink(source, target)
                .map_err(|e| CyonixError::CustomError(String::from("Failed to create symlink")))?;
        }
        Ok(())
    }
}
