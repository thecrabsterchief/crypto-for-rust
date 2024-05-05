mod testcases;

use crypto_for_rust::rc4::{encrypt_rc4, decrypt_rc4};
use testcases::{TestCase, gen_testcases};

#[test]
pub fn test_encryption() {
    let tests: Vec<TestCase> = gen_testcases();

    for test in tests {
        let enc = encrypt_rc4(&test.key, &test.plaintext);
        assert_eq!(enc, test.ciphertext);
    }
}

#[test]
pub fn test_decryption() {
    let tests: Vec<TestCase> = gen_testcases();

    for test in tests {
        let dec = decrypt_rc4(&test.key, &test.ciphertext);
        assert_eq!(dec, test.plaintext);
    }
}