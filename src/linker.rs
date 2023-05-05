use std::os::unix::fs;
use std::path::Path;
use crate::error::CyonixError;

pub struct Linker {
    path: String,
}

impl Linker {
    pub fn new(path: &str) -> Linker {
        Linker {
            path: path.to_string(),
        }
    }

    pub fn is_symlink(&self) -> bool {
        std::path::Path::new(&self.path).is_symlink()
    }

    pub fn create_symlink(&self, file: Vec<(&str, &str)>) -> Result<(), CyonixError> {
        if !self.is_symlink(){
            for (name, location) in file {
                let source = Path::new(&self.path).join(name);
                let target = Path::new(location);

                if target.exists() {
                    return Err(CyonixError::SpecificError(format!("Target file already exists: {}", location)));
                }

                fs::symlink(source, target)?;
            }
        }
        Ok(())
    }
}
