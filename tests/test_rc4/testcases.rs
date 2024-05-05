pub struct TestCase {
    pub key       : Vec<u8>,
    pub plaintext : Vec<u8>,
    pub ciphertext: Vec<u8>,
}

pub fn gen_testcases() -> Vec<TestCase> {
    vec![
        TestCase {
            key       : vec![0x24, 0x8b, 0x2d, 0xb2, 0x69, 0xe3, 0x47, 0x35, 0xed, 0xa7, 0x97, 0x32, 0xd3, 0x21, 0xc9, 0x63, 0x8a, 0x59, 0x27, 0x3e, 0xca, 0x8a, 0x3c, 0xc8, 0xb3, 0xe2, 0xcd, 0x19, 0x87],
            plaintext : vec![0xd6, 0x25, 0x2b, 0x18, 0x1c, 0x83, 0xa8, 0xa1, 0x1, 0x89, 0x7f, 0xf, 0xa8, 0xd2, 0x36, 0xe9, 0x3c, 0xfa, 0x36, 0x79, 0xb5],
            ciphertext: vec![0xe8, 0x69, 0x81, 0x4c, 0x9e, 0xd9, 0xeb, 0x5, 0x4c, 0x17, 0x11, 0x8f, 0x19, 0xd8, 0xf3, 0x61, 0x4d, 0x16, 0xc1, 0xf5, 0x10]
        },
        TestCase {
            key       : vec![0x78, 0x2a, 0x94, 0xfa, 0x57, 0x55, 0x68, 0xcb, 0xc2, 0x84, 0x91, 0x5d, 0xea, 0x6d, 0x47, 0xbc, 0xd5, 0x3, 0xa5, 0x3b, 0x1f, 0xea, 0x63, 0x3c],
            plaintext : vec![0xa2, 0x6a, 0xfc, 0xd2, 0x4d, 0xeb, 0xfb, 0xc2, 0x50, 0x7b, 0xc7, 0x23, 0xfc, 0xcb, 0xc8, 0xf3],
            ciphertext: vec![0x31, 0xea, 0xe6, 0xa2, 0x6d, 0x6d, 0x73, 0x8d, 0xc6, 0xa4, 0x81, 0xcd, 0x5, 0xfe, 0x28, 0x42]
        },
        TestCase {
            key       : vec![0xeb, 0x86, 0x7c, 0x6, 0xec, 0xa1, 0x4, 0xfc, 0x51, 0x51, 0xcf, 0xcc, 0x3e, 0x95, 0xfd, 0x4a, 0x53, 0xad, 0x3d, 0x6d, 0x9f, 0x87],
            plaintext : vec![0xd2, 0x86, 0xa5, 0xce, 0xf2, 0xdd, 0x21, 0xaa, 0x1b, 0xb8, 0x77, 0xc9, 0x2e, 0x2, 0x8e, 0x9b, 0x65, 0xb5, 0x2b, 0xcd, 0xd5, 0x91, 0x50, 0x60, 0x3a, 0x17, 0x66, 0xa6, 0x9d, 0x73, 0x40, 0xc0],
            ciphertext: vec![0x2f, 0xd8, 0xd8, 0x59, 0x52, 0x23, 0xcf, 0x6d, 0xf8, 0xc9, 0xdd, 0xfe, 0xbc, 0x19, 0x1a, 0xb, 0x13, 0x29, 0xb6, 0xc4, 0xbf, 0xbb, 0x1a, 0xdf, 0xac, 0xfb, 0x84, 0xe3, 0xd3, 0x66, 0xd7, 0x3a]
        }
    ]
}