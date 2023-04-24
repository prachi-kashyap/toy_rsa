use rand::Rng;
use toy_rsa::{decrypt, encrypt, genkey};

#[test]
fn test_random_encrypt_decrypt() {
    let mut rng = rand::thread_rng();

    // Run the test multiple times to increase confidence in the implementation.
    for _ in 0..10 {
        let (p, q) = genkey();
        let public_key = p as u64 * q as u64;

        let msg: u32 = rng.gen_range(1..u32::MAX);
        let encrypted = encrypt(public_key, msg);
        let decrypted = decrypt((p, q), encrypted);

        assert_eq!(
            msg, decrypted,
            "Decrypted message should match the original message"
        );
    }
}

#[test]
fn test_fixed_key_random_messages() {
    let p: u32 = 0xed23e6cd;
    let q: u32 = 0xf050a04d;
    let public_key = p as u64 * q as u64;

    let mut rng = rand::thread_rng();

    // Test the encryption and decryption with a fixed key pair and several random messages.
    for _ in 0..10 {
        let msg: u32 = rng.gen_range(1..u32::MAX);
        let encrypted = encrypt(public_key, msg);
        let decrypted = decrypt((p, q), encrypted);

        assert_eq!(
            msg, decrypted,
            "Decrypted message should match the original message for fixed key pair"
        );
    }
}
