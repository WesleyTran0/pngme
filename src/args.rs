use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Encodes the file by adding a Chunk with the given ChunkType and message
    Encode(EncodeParams),
    Decode(DecodeParams),
    Remove(RemoveParams),
    /// Prints the hidden message stored within the hidden file
    Print(PrintParams),
}

impl Cli {
    /// Processes this command
    pub fn process_command(&self) {
        match &self.command {
            Commands::Encode(params) => {
                println!("Encode was called with these params: {params:?}");
            }
            Commands::Decode(params) => {
                println!("Decode was called with these params: {params:?}");
            }
            Commands::Remove(params) => {
                println!("Remove was called with these params: {params:?}");
            }
            Commands::Print(params) => {
                println!("Print was called with these params: {params:?}");
            }
        }
    }
}

#[derive(Args, Debug)]
struct EncodeParams {
    path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct DecodeParams {
    path: PathBuf,
    chunk_type: String,
}

#[derive(Args, Debug)]
struct RemoveParams {
    path: PathBuf,
    chunk_type: String,
}

#[derive(Args, Debug)]
struct PrintParams {
    path: PathBuf,
}
