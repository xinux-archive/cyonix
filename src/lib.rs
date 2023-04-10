pub mod config;
pub mod linker;
pub mod args;
pub mod error;
pub mod file;

use std::fmt::{Debug, Display};
use std::process::exit;
use config::Config;
use dirs::home_dir;
use crate::file::Files;


#[derive(Debug)]
pub struct Cyonix<'a> {
    pub config: Config<'a>,
    pub file: Files,
}

impl<'a> Default for Cyonix<'a> {
    fn default() -> Self {
        let location = home_dir().unwrap();

        Cyonix {
            config: Config::new(location.clone()),
            file: Files::new(location),
        }
    }
}

pub fn lemme_panic<T, E: Debug + Display>(result: Result<T, E>){
    if let Err(e) = result{
        println!("{e}");
        exit(1);
    }
}