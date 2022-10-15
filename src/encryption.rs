use byteorder::{BigEndian, WriteBytesExt};

use crate::errors::EncryptionError;

pub fn decrypt(encrypted: &mut Vec<u8>) -> String {
    let mut encrypted = (*encrypted.split_off(4)).to_vec();
    let mut key = 171;
    let mut next: u8;
    let len = encrypted.len();
    for item in encrypted.iter_mut().take(len) {
        next = *item;
        *item ^= key;
        key = next;
    }
    String::from_utf8_lossy(&encrypted).to_string()
}

pub fn encrypt(command: &str) -> Result<Vec<u8>, EncryptionError> {
    let len = command.len();
    let raw_bytes = command.as_bytes();
    let mut encrypted = vec![];
    let mut key = 171;
    let mut payload: Vec<u8> = Vec::with_capacity(len);
    #[allow(clippy::cast_possible_truncation)]
    let result = encrypted.write_u32::<BigEndian>(len as u32);
    if result.is_err() {
        return Err(EncryptionError::U32Error);
    }
    for i in 0..len {
        payload.push(raw_bytes[i] ^ key);
        key = payload[i];
    }
    for i in payload {
        let result = encrypted.write_u8(i);
        if result.is_err() {
            return Err(EncryptionError::U8Error);
        }
    }
    Ok(encrypted)
}
