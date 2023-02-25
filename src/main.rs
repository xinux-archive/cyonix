// use cyonix::Cyonix;

use clap::{Parser, Subcommand, ValueEnum};

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

    // let instance = Cyonix::new();

    match args.command {
        Commands::Add { file } => {
            println!("Cloning {file}");
            println!("{:?}", directories::UserDirs::new().unwrap().home_dir())
        }
    }
}
