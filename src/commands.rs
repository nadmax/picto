use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::picto::{DecodeArgs, EncodeArgs, ListArgs, RemoveArgs};
use crate::png::Png;
use crate::xor;
use crate::{Error, Result};

use std::fs::write;
use std::str::FromStr;

pub fn encode(args: EncodeArgs) -> Result<()> {
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let encoded_message = xor::xor_encode(args.message.as_bytes(), &args.key);
    let chunk = Chunk::new(chunk_type, encoded_message);
    let mut png = Png::from_file(&args.path)?;

    png.append_chunk(chunk);

    match args.output {
        Some(f) => write(f, png.as_bytes())?,
        None => write(args.path, png.as_bytes())?,
    };

    println!("Chunk type '{}' added", args.chunk_type);

    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(&args.path)?;
    let chunk = png.chunk_by_type(&args.chunk_type);

    match chunk {
        Some(c) => match xor::xor_decode(c.data(), &args.key) {
            Ok(decoded_message) => println!("Decoded message: {decoded_message}"),
            Err(err) => return Err(Error::new(err)),
        },
        None => println!("No message for Chunk type '{}'", args.chunk_type),
    }

    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.path)?;

    png.remove_chunk(&args.chunk_type)?;
    write(args.path, png.as_bytes())?;
    println!("Chunk type '{}' removed", args.chunk_type);

    Ok(())
}

pub fn list_chunks(args: ListArgs) -> Result<()> {
    let png = Png::from_file(&args.path)?;

    println!("{png}");

    Ok(())
}
