use clap::Parser;

use pngme::{
    args::{Cli, Commands},
    commands, Result,
};

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    match &cli.command {
        Commands::Encode(args) => commands::encode(args)?,
        Commands::Decode(args) => commands::decode(args)?,
        Commands::Print(args) => commands::print(args)?,
        Commands::Remove(args) => commands::remove(args)?,
    };
    Ok(())
}
