use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

pub struct Linker {
    path: String,
}

impl Linker {
    fn new(path: &str) -> Linker {
        Linker {
            path: path.to_string(),
        }
    }

    fn exists(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }

    fn create(&self) -> std::io::Result<()> {
        std::fs::create_dir(&self.path)
    }

    fn is_symlink(&self) -> bool {
        std::path::Path::new(&self.path).is_symlink()
    }

}
