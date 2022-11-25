use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "MK", version = "0.1.0", about = "Png steganography", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encode a Png with a message
    Encode(EncodeArgs),

    /// Decode a message from Png
    Decode(DecodeArgs),

    /// Remove a chunk from Png
    Remove(RemoveArgs),

    ///Print the Png
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    pub file_path: String,

    pub chunk_type: String,

    pub message: String,

    pub output_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    pub file_path: String,

    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub file_path: String,

    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    pub file_path: String,
}
