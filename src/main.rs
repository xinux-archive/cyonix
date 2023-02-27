// use cyonix::Cyonix;

use clap::{Parser, Subcommand, Args};

/// Dotfile farm manager
#[derive(Debug, Parser)]
#[command(name = "cyonix")]
#[command(about = "Dotfile farm manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Adding files to list
    Add {
        /// Path to file
        file: String,
    },
    
    /// Synchronize dotfiles with cloud
    Git(GitArgs),
    
    /// Restoring and managing dotfiles
    Restore {
        file: Option<String>
    }
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct GitArgs {
    #[command(subcommand)]
    command: GitsCommands,
}

#[derive(Debug, Subcommand)]
enum GitsCommands {
    /// Initialize git repo in dotfiles directory
    Init,
    
    /// Push dotfiles to git repo
    Push,
}


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
