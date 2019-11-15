use base64;
use rand::rngs::OsRng;

use openssl::rsa::{ Rsa, Padding, };
use openssl::symm::{ encrypt, Cipher, };
use rand::RngCore;
use urlqstring::querystring;

const iv: &str = "0102030405060708";
const preset_key: &str = "0CoJUm6Qyw8W8jud";
const base62: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const rsa_public_key: &str = "-----BEGIN PUBLIC KEY-----\nMIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ37BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvaklV8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44oncaTWz7OBGLbCiK45wIDAQAB\n-----END PUBLIC KEY-----";

pub struct Crypto;

impl Crypto {
    pub fn weapi(text: &str) -> String {
        let mut secret_key = [0u8; 16];
        OsRng.fill_bytes(&mut secret_key);
        let base_62 = base62.to_string().into_bytes();
        let key: Vec<u8> = secret_key.iter().map(|i| {
            base_62[ (i % 62) as usize ]
        }).collect();

        let params = Crypto::aes_encrypt(
            &Crypto::aes_encrypt(text, preset_key),
            std::str::from_utf8(&key).unwrap()
        );

        let enc_sec_key = Crypto::rsa_encrypt(
            std::str::from_utf8(&key.iter().rev().map(|n|*n).collect::<Vec<u8>>()).unwrap(),
            rsa_public_key
        );

        querystring::stringify(vec![
            ("params", &params),
            ("encSecKey", &enc_sec_key)
        ])
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
    use base64::CharacterSet::Crypt;

    #[test]
    fn test_aes_encrypt() {
        let msg = r#"{"ids":"[347230]","br":999000}"#;

        let key1 = "gLiwKFot44HYFRAy";

        let res = Crypto::aes_encrypt(msg,  preset_key);

        assert_eq!(res, "pgHP1O/hr+IboRMAq6HzpHjyYwNlv1x0G4BBjd1ohdM=");

        let res2 = Crypto::aes_encrypt(&res, key1);

        assert_eq!(res2, "3EC4ojigTl0OgjyYtcd+97P7YKarculWrOxSgNO5clkQftvO1jOvS8aAhK6diyOb");
    }

    #[test]
    fn test_rsa_encrypt() {
        let key2 = "yARFYH44toFKwiLg";
        let res = Crypto::rsa_encrypt(key2, rsa_public_key);
        assert_eq!(res, "5ff8bdb3ed3dd15a26e9025e9abcff0d7a3764dafbc70e33859a892584c681f1aab314b8ad1f3418650ff851bdb0685fc5136a88e059c592da104bbeaba666fbe89eb405c7b66eab4db8ee3ab13a3f98cb41b2ac9981ed4e441ed8e1870524d001ee6ebc1c09f7a945677e5b56a3e964a224c3ee75ac43fbf513f6a8bf7472ee");
    }
}
