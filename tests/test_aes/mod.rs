mod testcases;

use crypto_for_rust::aes::{encrypt_aes, decrypt_aes};
use testcases::{TestCase, gen_testcases};

#[test]
pub fn test_encryption() {
    let tests: Vec<TestCase> = gen_testcases();

    for test in tests {
        let enc = encrypt_aes(&test.key, &test.plaintext);
        assert_eq!(enc, test.ciphertext);
    }
}

#[test]
pub fn test_decryption() {
    let tests: Vec<TestCase> = gen_testcases();

    for test in tests {
        let dec = decrypt_aes(&test.key, &test.ciphertext);
        assert_eq!(dec, test.plaintext);
    }
}