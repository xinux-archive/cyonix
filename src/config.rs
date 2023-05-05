use std::fmt::Debug;
use std::io::ErrorKind;
use std::path::PathBuf;
use dirs::home_dir;
use crate::error::CyonixError;


#[cfg(target_os = "macos")]
pub const PATHWAY: &str = ".cyonix";

#[cfg(target_os = "windows")]
pub const PATHWAY: &str = "AppData/Roaming/cyonix";

#[cfg(target_os = "linux")]
pub const PATHWAY: &str = ".cyonix";

pub const CONFIG: &str = ".config";
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

    // TODO: implement add, save function
    // when user run add command, program should
    // add the file and its location to the vector files: Vec<(&'a str, &'a str)>
    // and when program finishes it's job (nearly the end of program), save the files
    // vector to the file.list file
    // write("{file} {location}", file, location)
    // Reminder!
    // Before writing to file, delete the old records or just overwrite
    // so you don't duplicate records


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

            if let Some(name) = words.next(){
                if let Some(location) = words.next(){
                    self.files.push((name, location)); // (".zshrc" , "~/.zshrc")
                }
            }
        }
    }

    pub fn read(&self, file: &'a str) -> Result<&'a str, CyonixError> {
        match home_dir() {
            Some(home_dir) => {
                let conf_file = home_dir.join(PATHWAY).join(file);
                if !conf_file.exists() {
                    return Err(CyonixError::CustomError(ErrorKind::AlreadyExists))
                }
                std::fs::read_to_string(conf_file)?;
                Ok(file)
            }
            None => Err(CyonixError::SpecificError(String::from("Failed to find the home directory"))),
        }
    }

    // TODO: implement function to bootstrap
    pub fn bootstrap() {}

    pub fn init(&mut self, file: &'a str) -> Result<(), CyonixError>{
        // TODO: check if file actually exists
        // If not, it's the first time user us running the program
        // So create, file and folders for the user

        // Read the file list and parse it
        // We read the content of file
        let file_list_content = self.read(file)?;

        // If content is empty, return error... Don't parse!
        if file_list_content.is_empty() {
            return Err(CyonixError::SpecificError(String::from("File list is empty")))
        }

        self.parse(file_list_content);
        Ok(())
    }
}
