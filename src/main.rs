mod config;
mod error;

use clap::Parser;
use cyonix::args::{Cli, Commands, GitCommands};
use cyonix::Cyonix;
use cyonix::error::CyonixError;

fn main() -> Result<(), CyonixError> {
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
