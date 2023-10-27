use std::fs::write;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::picto::{DecodeArgs, EncodeArgs, ListArgs, RemoveArgs};
use crate::png::Png;
use crate::xor;
use crate::Result;

pub fn encode(args: EncodeArgs) -> Result<()> {
    let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();
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
        Some(c) => {
            let decoded_message = xor::xor_decode(c.data(), &args.key);
            println!("Decoded message: {decoded_message}");
        }
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
