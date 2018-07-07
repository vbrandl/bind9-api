extern crate failure;
extern crate hex;
extern crate ring;

#[cfg(test)]
#[macro_use]
extern crate proptest;

use failure::Error;
use hex::{FromHex, ToHex};
use ring::{digest, hmac};

type Result<T> = std::result::Result<T, Error>;

pub fn bytes_to_hex_str(bytes: &[u8]) -> Result<String> {
    let mut output = String::new();
    bytes.write_hex(&mut output)?;
    Ok(output)
}

pub fn hex_str_to_bytes(hex_str: &str) -> Result<Vec<u8>> {
    Ok(Vec::from_hex(hex_str)?)
}

pub fn verify_signature(key: &[u8], msg: &[u8], signature: &[u8]) -> bool {
    let key = hmac::VerificationKey::new(&digest::SHA256, key);
    hmac::verify(&key, msg, signature)
        .map(|_| true)
        .unwrap_or(false)
}

pub fn sign(key: &[u8], msg: &[u8]) -> Vec<u8> {
    let key = hmac::SigningKey::new(&digest::SHA256, key);
    let signature = hmac::sign(&key, msg);
    signature.as_ref().into_iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        fn sign_verify(key: Vec<u8>, msg: Vec<u8>) {
            let sig = sign(&key, &msg);
            assert!(verify_signature(&key, &msg, &sig));
        }
    }

    proptest! {
        #[test]
        fn to_from_hex(data: Vec<u8>) {
            assert_eq!(hex_str_to_bytes(&bytes_to_hex_str(&data).unwrap()).unwrap(), data);
        }
    }
}
