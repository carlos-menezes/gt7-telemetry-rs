use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptError {
    #[error("invalid magic value")]
    InvalidMagicValue(),
}

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("unexpected magic value")]
    UnexpectedMagicValue(),
    #[error("error parsing packet")]
    UnexpectedSequence(#[from] io::Error),
    #[error("unknown flag")]
    UnknownFlag(),
}
