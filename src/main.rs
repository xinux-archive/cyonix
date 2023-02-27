// use cyonix::Cyonix;

use clap::{Parser, Subcommand, ValueEnum, Args};

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
    Git {
        init: bool,
        push: bool,
    },
    
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
    Push,
    Pop,
    Apply,
}


fn main() {
    let args = Cli::parse();
    
    match args.command {
        Commands::Add { file } => {
            println!("Cloning {file}");
        }
        Commands::Git { init, push } => {
            if init {
                println!("Initializing git repo");
            }
            if push {
                println!("Pushing to git repo");
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
