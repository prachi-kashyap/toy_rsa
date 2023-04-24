use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32) {
    loop {
        let p = rsa_prime();
        let q = rsa_prime();
        let lambda = lcm((p - 1).into(), (q - 1).into());
        if EXP < lambda && gcd(EXP, lambda) == 1 {
            return (p, q);
        }
    }
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64, EXP, key)
}

/// Decrypt the ciphertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let (p, q) = key;
    let n = (p as u64) * (q as u64);
    let lambda = lcm((p - 1).into(), (q - 1).into());
    let d = modinverse(EXP, lambda);
    modexp(msg, d, n) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toy_rsa() {
        let p = 0xed23e6cd;
        let q = 0xf050a04d;
        let n = 0xde9c5816141c8ba9;
        let msg = 0x12345f;
        let encrypted = 0x6418280e0c4d7675;
        let decrypted = 0x12345f;

        assert_eq!(encrypt(n, msg), encrypted);
        assert_eq!(decrypt((p, q), encrypted), decrypted);
    }
}
