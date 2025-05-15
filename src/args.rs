use crate::chunk::{Chunk, ParseChunkError};
use crate::chunk_type::{ChunkType, ParseChunkTypeError};
use crate::png::{ParsePngError, Png};
use clap::{Args, Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

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
    /// Decodes the first Chunk with the given ChunkType in the file
    Decode(DecodeParams),
    /// Removes the first Chunk with the given ChunkType in the file
    Remove(RemoveParams),
    /// Prints the hidden messages stored within the hidden file
    Print(PrintParams),
}

impl Cli {
    /// Processes this command
    pub fn process_command(&self) {
        match &self.command {
            Commands::Encode(params) => {
                params.process_command();
            }
            Commands::Decode(params) => {
                println!("Your Decoded Picture:\n{}", params.process_command());
            }
            Commands::Remove(params) => {
                params.process_command();
            }
            Commands::Print(params) => {
                println!("{}", params.process_command());
            }
        }
    }
}

// #[derive(Debug)]
// enum PngmeError {
//     ParseChunkTypeError,
//     ParseChunkError,
//     ParsePngError,
// }

/// Params taht can be passed into the encode command
#[derive(Args, Debug)]
struct EncodeParams {
    path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
}

/// Functions that use the Encode paramters to do something
impl EncodeParams {
    /// Processes and performs the encode action using the given paramters
    fn process_command(&self) {
        let given_png_as_bytes = fs::read(&self.path).unwrap();
        let mut png = Png::try_from(given_png_as_bytes.as_slice()).unwrap();
        let chunk = Chunk::new(
            ChunkType::from_str(&self.chunk_type).unwrap(),
            self.message.as_bytes().to_vec(),
        );

        png.append_chunk(chunk);

        match &self.output_file {
            Some(out_path) => fs::write(out_path, png.as_bytes()).unwrap(),
            None => fs::write(&self.path, png.as_bytes()).unwrap(),
        }
    }
}

/// Params that can be passed into the Decode command
#[derive(Args, Debug)]
struct DecodeParams {
    path: PathBuf,
    chunk_type: String,
}

/// Functions that use the Decode parameters to do something
impl DecodeParams {
    /// Processes and performs the decode action using the given parameters
    fn process_command(&self) -> String {
        let png_as_bytes = fs::read(&self.path).unwrap();
        let png = Png::try_from(png_as_bytes.as_slice()).unwrap();
        let decoded_chunk = png.chunk_by_type(&self.chunk_type).unwrap();

        decoded_chunk.data_as_string().unwrap()
    }
}

/// Holds the paramters for the Remove command
#[derive(Args, Debug)]
struct RemoveParams {
    path: PathBuf,
    chunk_type: String,
}

/// Functions that use the Remove paramters to do something
impl RemoveParams {
    /// processes and performs the remove action using the given parameters
    fn process_command(&self) {
        let png_as_bytes = fs::read(&self.path).unwrap();
        let mut png = Png::try_from(png_as_bytes.as_slice()).unwrap();

        png.remove_first_chunk(&self.chunk_type).unwrap();
        fs::write(&self.path, png.as_bytes()).unwrap();
    }
}

/// Holds the parameters for the Print command
#[derive(Args, Debug)]
struct PrintParams {
    path: PathBuf,
}

/// Functions taht use the Print paramters to do something
impl PrintParams {
    /// processes and performs the print action using the given paramters
    fn process_command(&self) -> String {
        let png_as_bytes = fs::read(&self.path).unwrap();
        let png = Png::try_from(png_as_bytes.as_slice()).unwrap();

        format!("{}", png)
    }
}
