use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    List(ListArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub key: String,
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
    pub key: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    pub path: PathBuf,
}
