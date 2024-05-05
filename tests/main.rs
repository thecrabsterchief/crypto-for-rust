mod test_rc4;
mod test_aes;

fn main() {
    // test RC4
    test_rc4::test_encryption();
    test_rc4::test_decryption();

    // test AES
    test_aes::test_decryption();
    test_aes::test_encryption();
}