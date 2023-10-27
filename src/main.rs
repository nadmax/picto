mod chunk;
mod chunk_type;
mod commands;
mod picto;
mod png;
mod xor;

use clap::Parser;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T, Error>;

fn main() -> Result<()> {
    let cli = picto::Cli::parse();

    match cli.command {
        picto::Commands::Encode(args) => commands::encode(args),
        picto::Commands::Decode(args) => commands::decode(args),
        picto::Commands::List(args) => commands::list_chunks(args),
        picto::Commands::Remove(args) => commands::remove(args),
    }
    .unwrap_or_else(|e| eprintln!("{e}"));

    Ok(())
}
