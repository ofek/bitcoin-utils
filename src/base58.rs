extern crate num;
extern crate phf;

pub use crypto::double_sha256_checksum;

use self::num::{BigUint, FromPrimitive, ToPrimitive};

const ALPHABET: &'static [u8; 58] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// Use ALPHABET_MAP: phf::Map<char, u8>
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));


pub fn b58encode(bytes: &[u8]) -> String {
    let mut encoded = Vec::with_capacity(bytes.len() * 256 / 58);
    let mut number = BigUint::from_bytes_be(&bytes);
    let alphabet_length = BigUint::from_u8(58u8).unwrap();
    let zero = BigUint::from_u8(0u8).unwrap();

    while number > zero {
        let remainder = &number % &alphabet_length;
        encoded.push(ALPHABET[remainder.to_usize().unwrap()]);
        number = &number / &alphabet_length;
    }

    for byte in bytes.iter() {
        if byte == &0u8 {
            encoded.push(49u8);
        } else {
            break;
        }
    }

    encoded.reverse();
    unsafe {
        String::from_utf8_unchecked(encoded)
    }
}


pub fn b58encode_check(bytes: &[u8]) -> String {
    let mut checked = bytes.to_vec();
    checked.extend_from_slice(&double_sha256_checksum(bytes));
    b58encode(&checked)
}
