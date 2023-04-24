use toy_rsa::{decrypt, encrypt, genkey};

fn main() {
    // Generate a pair of RSA primes
    let (p, q) = genkey();
    println!("Private Key: p = {:#x} q = {:#x}", p, q);

    // Calculate the public key
    let public_key = (p as u64) * (q as u64);
    println!("Public Key: p * q = {:#x}", public_key);

    // Sample message to encrypt and decrypt
    let message: u32 = 0x12345f;
    println!("Message: {:#x}", message);

    // Encrypt the message using the public key
    let encrypted = encrypt(public_key, message);
    println!("Encrypted: {:#x}", encrypted);

    // Decrypt the message using the private key (p, q)
    let decrypted = decrypt((p, q), encrypted);
    println!("Decrypted: {:#x}", decrypted);
}
