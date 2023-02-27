use clap::Parser;
use cyonix::args::{Cli, Commands, GitsCommands};

fn main() {
    let args = Cli::parse();
    
    match args.command {
        Commands::Add { file } => {
            println!("Cloning {file}");
        }
        Commands::Git(git) => {
            match git.command {
                GitsCommands::Init => {
                    println!("Initializing git repo");
                }
                GitsCommands::Push => {
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
}
