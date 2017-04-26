extern crate ring;

use self::ring::digest;


pub fn sha256(data: &[u8]) -> Vec<u8> {
    digest::digest(&digest::SHA256, data).as_ref().to_vec()
}


pub fn double_sha256(data: &[u8]) -> Vec<u8> {
    digest::digest(
        &digest::SHA256,
        digest::digest(&digest::SHA256, data).as_ref()
    ).as_ref().to_vec()
}


pub fn double_sha256_checksum(data: &[u8]) -> Vec<u8> {
    digest::digest(
        &digest::SHA256,
        digest::digest(&digest::SHA256, data).as_ref()
    ).as_ref()[0..4].to_vec()
}
