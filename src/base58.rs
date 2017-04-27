extern crate num;
extern crate phf;

use self::num::{Integer, BigUint, FromPrimitive, ToPrimitive};

pub use crypto::double_sha256_checksum;
pub use errors::Base58DecodeError;

const ALPHABET: &'static [u8; 58] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// Use ALPHABET_MAP: phf::Map<char, u8>
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));


pub fn b58encode(bytestr: &[u8]) -> String {
    let mut encoded = Vec::with_capacity(bytestr.len() * 256 / 58);
    let mut number = BigUint::from_bytes_be(&bytestr);
    let alphabet_length = BigUint::from_u8(58u8).unwrap();
    let zero = BigUint::from_u8(0u8).unwrap();

    while number > zero {
        let (quotient, remainder) = number.div_rem(&alphabet_length);
        number = quotient;
        encoded.push(ALPHABET[remainder.to_usize().unwrap()]);
    }

    for byte in bytestr.iter() {
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


pub fn b58encode_check(bytestr: &[u8]) -> String {
    let mut checked = bytestr.to_vec();
    checked.extend_from_slice(&double_sha256_checksum(bytestr));
    b58encode(&checked)
}


pub fn b58decode(string: &String) -> Result<Vec<u8>, Base58DecodeError> {
    let mut number = BigUint::new(Vec::new());
    let alphabet_length = BigUint::from_u8(58u8).unwrap();

    for c in string.chars() {
        number = &number * &alphabet_length;
        let index = match ALPHABET_MAP.get(&c) {
            Some(v) => v,
            None => return Err(
                Base58DecodeError::InvalidCharacter {character: c}
            )
        };
        number = &number + BigUint::from_u8(*index).unwrap();
    }

    let mut bytestr = Vec::new();

    for c in string.chars() {
        if c == '1' {
            bytestr.push(0u8);
        } else {
            break;
        }
    }

    bytestr.extend_from_slice(&number.to_bytes_be());
    Ok(bytestr)
}


pub fn b58decode_check(string: &String) -> Result<Vec<u8>, Base58DecodeError> {
    let mut decoded = b58decode(string).unwrap();
    let length = &decoded.len();
    let decoded_checksum: Vec<_> = decoded.drain(length - 4 ..).collect();
    let hash_checksum = double_sha256_checksum(&decoded);

    if &decoded_checksum != &hash_checksum {
        return Err(
            Base58DecodeError::IncorrectChecksum {
                decoded_checksum: decoded_checksum,
                correct_checksum: hash_checksum
            }
        );
    }

    Ok(decoded)
}
