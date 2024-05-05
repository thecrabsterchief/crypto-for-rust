mod utils;
use utils::AES;

pub fn encrypt_aes(key: &Vec<u8>, plaintext: &Vec<u8>) -> Vec<u8> {
    let mut aes = AES::new(&key);
    aes.encrypt_block(plaintext)
}

pub fn decrypt_aes(key: &Vec<u8>, ciphertext: &Vec<u8>) -> Vec<u8> {
    let mut aes = AES::new(&key);
    aes.decrypt_block(ciphertext)
}