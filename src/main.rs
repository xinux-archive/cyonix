mod config;
mod error;

use clap::Parser;
use cyonix::args::{Cli, Commands, GitCommands};
use cyonix::Cyonix;
use cyonix::error::CyonixError;
use cyonix::config::{Config, config_directory, FILE};

fn main() -> Result<(), CyonixError> {
    let path = config_directory();
    let mut config = Config::new(&path);

    // Read the file list and parse it
    let file_list = config.read(FILE)?;
    config.parse(file_list);

    // Create the symlinks
    config.create_symlinks().expect("Failed to create symlinks");

    let args = Cli::parse();
    let cyonix: Cyonix = Default::default();
    
    match args.command {
        Commands::Add { file } => {
            println!("Cloning {file}");
            cyonix.move_file(&file)?;
        },
        Commands::Delete { file} => {
            println!("Deleting {file}");
            cyonix.delete(&file)?;
        }
        Commands::Git(git) => {
            match git.command {
                GitCommands::Init => {
                    println!("Initializing git repo");
                }
                GitCommands::Push => {
                    println!("Pushing dotfiles to git repo");
                }
            }
        }
        Commands::Restore { file } => {
            if let Some(file) = file {
                println!("Restoring {file}");
            } else {
                println!("Restoring all files");
            }
        }
    }
    Ok(())
}
