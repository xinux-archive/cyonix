use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let main = format!("../{}", ".cyonix");
    let submain = format!("../{}", ".cyonix/storage");
    fs::create_dir(main)?;
    fs::create_dir(submain)?;
    let mut file = File::create("../.cyonix/file_list")?;

    Ok(())
}
