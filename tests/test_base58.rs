extern crate bitcoin_utils;

use bitcoin_utils::base58::{
    b58decode, b58encode, b58encode_check
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b58encode() {
        assert_eq!(
            b58encode(b"\x00\x92F\x1b\xdeb\x83\xb4a\xec\xe7\xdd\xf4\xdb\xf1\xe0\xa4\x8b\xd1\x13\xd8&E\xb4\xbf"),
            String::from("1ELReFsTCUY2mfaDTy32qxYiT49z786eFg")
        );
    }

    #[test]
    fn test_b58encode_check() {
        assert_eq!(
            b58encode_check(b"\x00\x92F\x1b\xdeb\x83\xb4a\xec\xe7\xdd\xf4\xdb\xf1\xe0\xa4\x8b\xd1\x13\xd8"),
            String::from("1ELReFsTCUY2mfaDTy32qxYiT49z786eFg")
        );
    }

    #[test]
    fn test_b58decode() {
        assert_eq!(
            b58decode(&String::from("1ELReFsTCUY2mfaDTy32qxYiT49z786eFg")).unwrap(),
            b"\x00\x92F\x1b\xdeb\x83\xb4a\xec\xe7\xdd\xf4\xdb\xf1\xe0\xa4\x8b\xd1\x13\xd8&E\xb4\xbf"
        );
    }
}
