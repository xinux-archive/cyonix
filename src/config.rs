use std::fmt::Debug;
use std::io::Read;
use std::path::Path;

#[cfg(target_os = "macos")]
pub const PATHWAY: &str = ".cyonix";

#[cfg(target_os = "windows")]
pub const PATHWAY: &str = "AppData/Roaming/cyonix";

#[cfg(target_os = "linux")]
pub const PATHWAY: &str = ".cyonix";

const STORAGE: &str = "/storage";
const FILE: &str = "/file.list";

#[derive(Debug, Clone)]
pub struct Config {
    files: Vec<&'static str>,
}

/// FILE.LIST
///
/// .zshrc   ~/.zshrc
/// .bashrc  ~/.bashrc
///       ^     ^
///       |     |
/// Vec<(&str,&str)>

impl Config {
    pub fn new(path: &Path) -> Config {
        let path = Path::new(
          format!("{}/{}/{}",
                  path.to_str().unwrap(),
                  PATHWAY,
                  FILE
          ).as_str());
        
        let mut list = String::new();
        
        // read file
        std::fs::File::open(path)
            .unwrap()
            .read_to_string(&mut list)
            .expect("Couldn't stream to buf");
      
      

        Config { files: Vec::new() }
    }
}
