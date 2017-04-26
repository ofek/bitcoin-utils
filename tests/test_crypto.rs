extern crate bitcoin_utils;

use bitcoin_utils::crypto::{
    sha256, double_sha256, double_sha256_checksum
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        assert_eq!(
            sha256(b"data"),
            b":n\xb0y\x0f9\xac\x87\xc9O8V\xb2\xdd,]\x11\x0eh\x11`\"a\xa9\xa9#\xd3\xbb#\xad\xc8\xb7"
        );
    }

    #[test]
    fn test_double_sha256() {
        assert_eq!(
            double_sha256(b"data"),
            b"FDr\xb5`y\xde\xd3\xd3Y\xb1y5bK\xdb\x84\x87\xb6\xa6HV\t\x07%'}\xdb_\xb5Wj"
        );
    }

    #[test]
    fn test_double_sha256_checksum() {
        assert_eq!(
            double_sha256_checksum(b"data"),
            b"FDr\xb5"
        );
    }
}
