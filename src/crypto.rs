use base64;
use rand::rngs::OsRng;

use openssl::rsa::{ Rsa, Padding, };
use openssl::symm::{ encrypt, Cipher, };
use rand::RngCore;

const iv: &str = "0102030405060708";
const preset_key: &str = "0CoJUm6Qyw8W8jud";
const base62: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const rsa_public_key: &str = "-----BEGIN PUBLIC KEY-----\nMIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ37BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvaklV8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44oncaTWz7OBGLbCiK45wIDAQAB\n-----END PUBLIC KEY-----";

pub struct Crypto;

impl Crypto {
    pub fn weapi(text: &str) ->(String, String) {
        let mut secret_key = [0u8; 16];
        OsRng.fill_bytes(&mut secret_key);
        let base_62 = base62.to_string().into_bytes();
        let key: Vec<u8> = secret_key.iter().map(|i| {
            base_62[ (i % 62) as usize ]
        }).collect();

        let params = Crypto::aes_encrypt(
            &Crypto::aes_encrypt(text, preset_key),
            std::str::from_utf8(&key).unwrap(),
        );

        let enc_sec_key = Crypto::rsa_encrypt(
            std::str::from_utf8(&key.iter().rev().map(|n|*n).collect::<Vec<u8>>()).unwrap(),
            rsa_public_key
        );

        (params, enc_sec_key)
    }

    pub fn aes_encrypt(data: &str, key: &str) -> String {
        let cipher = Cipher::aes_128_cbc();
        let cipher_text = encrypt(
            cipher,
            key.as_bytes(),
            Some(iv.as_bytes()),
            data.as_bytes()
        ).unwrap();

        base64::encode(&cipher_text)
    }

    pub fn rsa_encrypt(data: &str, key: &str) -> String {
        let rsa = Rsa::public_key_from_pem(key.as_bytes()).unwrap();

        let prefix = vec![0u8; 128-data.len()];

        let data = [&prefix[..], &data.as_bytes()[..]].concat();

        let mut buf = vec![0; rsa.size() as usize];

        let encrypted_len = rsa.public_encrypt(&data, &mut buf, Padding::NONE).unwrap();

        hex::encode(buf)
    }
}

#[cfg(test)]
mod tests {

    use super::Crypto;
    use crate::crypto::{preset_key, rsa_public_key};

    #[test]
    fn test_aes_encrypt() {
        let msg = r#"{"ids":"[347230]","br":999000}"#;

        let res = Crypto::aes_encrypt(msg,  preset_key);

        assert_eq!(res, "pgHP1O/hr+IboRMAq6HzpHjyYwNlv1x0G4BBjd1ohdM=");
    }

    #[test]
    fn test_rsa_encrypt() {
        let msg = "hello world!";
        let res = Crypto::rsa_encrypt(msg, rsa_public_key);
        assert_eq!(res, "7979be697beebc8735f1693a4f76a4dd4f52bae1d5fb923e07dfa70565270528da94123b9a988fde402f66fc27e66c2deb456d9581e495f73e238f83f8f7bb7127bf9230def9b35385b43b2f08c32e00f4435af69c4d3a2b3d4e327343fcccd662abad1057a4a6262828863bb6bf14f6af8f972b22a3c58e47683541772a19ae");
    }
}
