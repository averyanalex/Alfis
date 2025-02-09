use std::fmt;
use std::fmt::{Debug, Formatter};

use chacha20poly1305::aead::{Aead, Error, NewAead};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

pub const ZERO_NONCE: [u8; 12] = [0u8; 12];

/// A small wrap-up to use Chacha20 encryption for domain names.
#[derive(Clone)]
pub struct Chacha {
    cipher: ChaCha20Poly1305,
    nonce: [u8; 12]
}

impl Chacha {
    pub fn new(key: &[u8], nonce: &[u8]) -> Self {
        let key = Key::from_slice(key);
        let cipher = ChaCha20Poly1305::new(key);
        let mut buf = [0u8; 12];
        buf.copy_from_slice(nonce);
        Chacha { cipher, nonce: buf }
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let nonce = Nonce::from(self.nonce);
        self.cipher.encrypt(&nonce, data.as_ref())
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let nonce = Nonce::from(self.nonce);
        self.cipher.decrypt(&nonce, data.as_ref())
    }

    pub fn get_nonce(&self) -> &[u8; 12] {
        &self.nonce
    }
}

impl Debug for Chacha {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str("ChaCha20Poly1305")
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::Chacha;
    use crate::to_hex;

    #[test]
    pub fn test_chacha() {
        let buf = b"178135D209C697625E3EC71DA5C760382E54936F824EE5083908DA66B14ECE18";
        let chacha1 = Chacha::new(b"178135D209C697625E3EC71DA5C76038", &buf[..12]);
        let bytes1 = chacha1.encrypt(b"TEST").unwrap();
        println!("{}", to_hex(&bytes1));

        let chacha2 = Chacha::new(b"178135D209C697625E3EC71DA5C76038", &buf[..12]);
        let bytes2 = chacha2.decrypt(&bytes1).unwrap();
        assert_eq!(String::from_utf8(bytes2).unwrap(), "TEST");

        let bytes2 = chacha2.encrypt(b"TEST").unwrap();

        assert_eq!(bytes1, bytes2);
    }
}