use clap::{Parser, Subcommand, Args};

/// Dotfile farm manager
#[derive(Debug, Parser)]
#[command(name = "cyonix")]
#[command(about = "Dotfile farm manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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
pub struct GitArgs {
    #[command(subcommand)]
    pub command: GitCommands,
}

#[derive(Debug, Subcommand)]
pub enum GitCommands {
    /// Initialize git repo in dotfiles directory
    Init,
    
    /// Push dotfiles to git repo
    Push,
}