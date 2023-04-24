# Toy RSA

**Author:** PRACHI KASHYAP

## Project Overview

This project implements a toy RSA library in Rust for educational purposes. The library provides key generation, encryption, and decryption functions based on the RSA algorithm. Please note that this implementation is not suitable for real-world cryptographic applications.

## Implementation

The library exposes the following functions:

- `genkey() -> (u32, u32)`: Generates a pair of primes in the range `2**31..2**32` suitable for RSA encryption. This uses rsa_prime, lcm and gcd from the lib
- `encrypt(key: u64, msg: u32) -> u64`: Encrypts the plaintext `msg` using the RSA public `key` and returns the ciphertext. This uses modexp from the lib.
- `decrypt(key: (u32, u32), msg: u64) -> u32`: Decrypts the ciphertext `msg` using the RSA private `key` and returns the resulting plaintext.This uses modinverse and modexp from the lib.

## Challenges and Solutions

1. **Generating prime numbers:** Generating random prime numbers within a given range was a challenge. I initially used the `rand` crate, but later provided an alternative solution that generates deterministic prime numbers without using the `rand` crate.

2. **Handling `Option` type returned by `invmod` function:** I made a mistake in using the `unwrap` method directly on the `u64` type, but later corrected it by using a match expression to handle the `Option<u64>` returned by the `invmod` function.

## Testing

I tested the functionality of the toy RSA library by:

1. Writing unit tests with `#[test]` attribute for each function.
2. Tested generating a random message, encrypted it using a randomly generated public key, and then decrypted it using the corresponding private key. Then wrote the assert statement based on the match.
3. Tested using a fixed pair.
4. Both test functions generates 10 random messages.

## Conclusion

This project helped me understand the basics of the RSA algorithm and how to implement it in Rust. It was a valuable learning experience, and I'm now more confident in working with cryptography concepts and Rust programming.
