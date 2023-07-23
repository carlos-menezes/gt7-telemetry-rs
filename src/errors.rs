use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptError {
    #[error("invalid magic value")]
    InvalidMagicValue(),
}
