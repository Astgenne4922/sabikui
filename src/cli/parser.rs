use clap::Subcommand;
use std::path::PathBuf;

pub use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub input: Option<Inputs>,

    /// List all available hashing functions
    #[arg(short, long, default_value_t = false)]
    pub list: bool,
}

#[derive(Subcommand)]
pub enum Inputs {
    /// Hash a single file
    #[command(arg_required_else_help = true)]
    File {
        /// File path
        path: PathBuf,
    },
    /// Hash multiple files
    #[command(arg_required_else_help = true)]
    Files {
        /// File paths
        paths: Vec<PathBuf>,
    },
    /// Hash the contents of a directory
    #[command(arg_required_else_help = true)]
    Directory {
        /// Directory path
        path: PathBuf,
    },
    /// Searches files to hash with a given regex pattern
    #[command(arg_required_else_help = true)]
    Wildcard {
        /// Search pattern
        regex: String,
    },
}
