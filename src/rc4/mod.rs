#![allow(dead_code,unused)]
mod utils;
use utils::RC4;

pub fn encrypt_rc4(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let mut rc4 = RC4::new(key);
    let mut ciphertext = Vec::with_capacity(plaintext.len());

    for &byte in plaintext {
        ciphertext.push(byte ^ rc4.prga());
    }

    ciphertext   
}

// RC4 is a symmetric cipher, so the decryption is the same as the encryption
pub fn decrypt_rc4(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    encrypt_rc4(key, ciphertext)
}