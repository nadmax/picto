mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<anyhow::Error>;
pub type Result<T> = anyhow::Result<T, Error>;

fn main() -> Result<()> {
    todo!()
}
