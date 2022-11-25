mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use args::Cli;
use args::Commands;
use clap::Parser;

fn main() -> Result<()>{
    let cli: Cli = Cli::parse();
    match &cli.command{
        Commands::Encode(args) => commands::encode(args)?,
        Commands::Decode(args) => commands::decode(args)?,
        Commands::Print(args) => commands::print(args)?,
        Commands::Remove(args) => commands::remove(args)?,
    };
    Ok(())
}
