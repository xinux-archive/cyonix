
use std::fmt::Debug;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use dirs::home_dir;
use crate::error::CyonixError;


#[cfg(target_os = "macos")]
pub const PATHWAY: &str = ".cyonix";

#[cfg(target_os = "windows")]
pub const PATHWAY: &str = "AppData/Roaming/cyonix";

#[cfg(target_os = "linux")]
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
    fn parse(&mut self, file: &'a str) {
        let lines = file.lines();
        
        for line in lines {
            let mut words = line.split_whitespace();

            let name = words.next().unwrap();
            let location = words.next().unwrap();
            
            self.files.push((name, location));
        }
    }
  
     fn read(&self, file: &'a str) -> std::io::Result<&'a str> {
         let conf_file = config_directory().join(file);
         if !conf_file.exists(){
             CyonixError::io_error(ErrorKind::NotFound, "");
         }

         std::fs::read_to_string(conf_file).expect("Failed to read :(");

         Ok(file)
    }
}
