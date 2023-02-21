use std::path::Path;

mod linker;

#[cfg(target_os = "macos")]
pub const PATHWAY: &str = "/.cyonix";

#[cfg(target_os = "windows")]
pub const PATHWAY: &str = "AppData/Roaming/cyonix";

#[cfg(target_os = "linux")]
pub const PATHWAY: &str = "/.cyonix";

// concat!(home.path() + PATHWAY);

#[derive(Default)]
struct Config {
  files: Vec<&'static str>
}

impl Config {
  pub fn new() -> Config {
    Config {
      files: Vec::new()
    }
  }
}

#[derive(Default)]
pub struct Cyonix {
  config: Config
}

impl Cyonix {
  pub fn new() -> Cyonix {
    Cyonix {
      config: Config::new()
    }
  }
  
  // move files to .cyonix
  pub fn move_file(&self, file: &str) {
    let path: &Path = Path::new(file);
    
    if path.exists() {
      std::fs::copy(file, format!("{}{}", PATHWAY, file)).expect("Failed to copy file");
      std::fs::remove_file(file).expect("Failed to remove file");
    }
    
    
  }
  
  pub fn add(&self, file: &str) {
    self.move_file(file);
  }
}