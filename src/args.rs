use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "MK", version = "0.1.0", about = "Png steganography", long_about = None)]
pub struct PngMeArgs {
    #[clap(subcommand)]
    pub sub_command: PngSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum PngSubCommand {
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
    file_path: String,

    chunk_type: String,

    message: String,

    output_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    file_path: String,

    chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    file_path: String,

    chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    file_path: String,
}
