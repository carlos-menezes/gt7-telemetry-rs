use salsa20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use salsa20::Salsa20;

use crate::errors::CryptError;
use crate::packet::PACKET_SIZE;

const KEY: &[u8] = b"Simulator Interface Packet GT7 ver 0.0";
pub const MAGIC_VALUE: u32 = 0x47375330;

pub struct Crypt {}

pub trait Cryptable {
    fn decrypt(bytes: &[u8; PACKET_SIZE]) -> Result<[u8; PACKET_SIZE], CryptError>;
}

impl Cryptable for Crypt {
    fn decrypt(bytes: &[u8; PACKET_SIZE]) -> Result<[u8; PACKET_SIZE], CryptError> {
        // Extract original initialization vector (IV)
        // We can unwrap here because the Ok variant will always be returned
        let original_init_vector: [u8; 4] = bytes[0x40..0x44].try_into().unwrap();

        // Combine the IV with a constant value
        let iv1 = u32::from_le_bytes(original_init_vector);
        let iv2 = iv1 ^ 0xDEADBEAF;
        let mut iv: [u8; 8] = [0u8; 8];
        iv[0..4].copy_from_slice(&iv2.to_le_bytes());
        iv[4..].copy_from_slice(&iv1.to_le_bytes());

        // Decrypt the packet using Salsa20 cipher
        let mut cipher = Salsa20::new(KEY[0..32].into(), &iv.into());
        let mut decrypted_buf = bytes.clone();
        cipher.apply_keystream(&mut decrypted_buf);

        // Check the magic value to validate decryption
        let magic = u32::from_le_bytes(decrypted_buf[0x0..0x4].try_into().unwrap());
        if magic != MAGIC_VALUE {
            return Err(CryptError::InvalidMagicValue());
        } else {
            Ok(decrypted_buf)
        }
    }
}
