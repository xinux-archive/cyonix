use std::fmt::Debug;
use std::io::Read;
use std::path::{Path, PathBuf};
use directories::BaseDirs;

#[cfg(target_os = "macos")]
pub const PATHWAY: &str = ".cyonix";

#[cfg(target_os = "windows")]
pub const PATHWAY: &str = "AppData/Roaming/cyonix";

#[cfg(target_os = "linux")]
pub const PATHWAY: &str = ".cyonix";

const STORAGE: &str = "/storage";
const FILE: &str = "/file.list";

/// Find PathBuf of config directory
fn config_directory(base_dirs: &BaseDirs) -> PathBuf {
    return base_dirs.home_dir().join(".config");
}

/// Find PathBuf of base home directory
fn base_directory() -> BaseDirs {
    return directories::BaseDirs::new()
        .expect("Could not find base directory");
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
    pub fn new(path: &Path) -> Config {
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
        let mut lines = file.lines();
        
        while let Some(line) = lines.next() {
            let mut words = line.split_whitespace();
            
            let name = words.next().unwrap();
            let location = words.next().unwrap();
            
            self.files.push((name, location));
        }
    }
  
    fn read() {
    
    }
}
