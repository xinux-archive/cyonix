// use cyonix::Cyonix;

use clap::{Arg, Command, Parser, Subcommand, ValueEnum};
use std::fs;

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

    let instance = Command::new("cyonix")
        .version("1.0.0")
        .author("Phoenixifier")
        .about(".file manager")
        .subcommand(Command::new("add"))
            .about("add files to storage");

    match instance.get_matches().subcommand_name() {
        Some("add") =>  match args.command {
            Commands::Add { file } => {
                println!("Cloning {file}");
                println!("{:?}", home::home_dir());
            }
        }

        _ => {
            eprintln!("No subcommand provided");
            std::process::exit(1);
        }
    }
}
