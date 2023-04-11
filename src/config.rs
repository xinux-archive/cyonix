use std::fmt::Debug;
use std::path::PathBuf;
use dirs::home_dir;
use crate::error::CyonixError;


#[cfg(target_os = "macos")]
pub const PATHWAY: &str = ".cyonix";

#[cfg(target_os = "windows")]
pub const PATHWAY: &str = "AppData/Roaming/cyonix";

#[cfg(target_os = "linux")]
pub const CONFIG: &str = ".file";
pub const PATHWAY: &str = ".cyonix";
pub const STORAGE: &str = "/storage";
pub const FILE: &str = "file.list";

#[derive(Debug, Clone)]
pub struct Config<'a> {
    /// Location of config folder
    home: PathBuf,
    
    /// List of files to be linked
    files: Vec<(&'a str, &'a str)>,
}

impl <'a> Config<'a> {
    /// Load instance of Config
    /// Temporarily left default configs, will be changed soon!
    pub fn new(path: PathBuf) -> Config<'a> {
      // here
        Config {
        home: Config::find_config(),
        files: Vec::new(),
      }
    }

    pub fn find_config() -> PathBuf {
        let base_dir = home_dir().unwrap();
        base_dir.join(CONFIG)
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
        match home_dir() {
            Some(home_dir) => {
                let conf_file = home_dir.join(PATHWAY).join(file);
                if !conf_file.exists() {
                    return Err(CyonixError::SpecificError(String::from("Failed to initialize config :(")))
                }
                std::fs::read_to_string(conf_file)?;
                Ok(file)
            }
            None => Err(CyonixError::SpecificError(String::from("Failed to find the home directory"))),
        }
    }

    pub fn init(&mut self, file: &'a str) -> Result<(), CyonixError>{
        // Read the file list and parse it
        let file_list = self.read(file)?;
        self.parse(file_list);
        Ok(())
    }
}
