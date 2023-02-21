use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

use cyonix::Cyonix;

use clap::{Args, Parser, Subcommand, ValueEnum};

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
    
    // Git {
    //     init: bool,
    //
    //     push: bool,
    // }
}

fn main() {
    let args = Cli::parse();

    // TODO (@phoenixifier & @katsuki-yuri): Create config parser
    
    let instance = Cyonix::new();
    
    match args.command {
        Commands::Add { file } => {
            println!("Cloning {file}");
        }
    }
}