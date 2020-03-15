
use base64;
use lazy_static::lazy_static;
use openssl::rsa::{ Rsa, Padding, };
use openssl::symm::{ encrypt, Cipher, };
use openssl::hash::{hash, MessageDigest, DigestBytes};
use rand::RngCore;
use rand::rngs::OsRng;
use urlqstring::QueryParams;
use crate::crypto::AesMode::{cbc, ecb};

lazy_static!{
    static ref IV: Vec<u8> = "0102030405060708".as_bytes().to_vec();
    static ref PRESET_KEY: Vec<u8> = "0CoJUm6Qyw8W8jud".as_bytes().to_vec();
    static ref LINUX_API_KEY: Vec<u8> = "rFgB&h#%2?^eDg:Q".as_bytes().to_vec();
    static ref BASE62: Vec<u8> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes().to_vec();
    static ref RSA_PUBLIC_KEY: Vec<u8> = "-----BEGIN PUBLIC KEY-----\nMIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ37BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvaklV8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44oncaTWz7OBGLbCiK45wIDAQAB\n-----END PUBLIC KEY-----".as_bytes().to_vec();
    static ref EAPIKEY: Vec<u8> = "e82ckenh8dichen8".as_bytes().to_vec();
}

#[allow(non_snake_case)]
pub struct Crypto;

#[allow(non_camel_case_types)]
pub enum HashType {
    md5
}

#[allow(non_camel_case_types)]
pub enum AesMode {
    cbc,
    ecb,
}

impl Crypto {
    pub fn hex_random_bytes(n: usize) -> String {
        let mut data: Vec<u8> = Vec::with_capacity(n);
        OsRng.fill_bytes(&mut data);
        hex::encode(data)
    }

    pub fn eapi(url: &str, text: &str) -> String {
        let message = format!( "nobody{}use{}md5forencrypt", url, text );
        let digest = hex::encode(hash(MessageDigest::md5(), message.as_bytes()).unwrap());
        let data = format!( "{}-36cd479b6b5-{}-36cd479b6b5-{}", url, text, digest );
        let params = Crypto::aes_encrypt(
            &data,
            &*EAPIKEY,
            ecb,
            Some(&*IV),
            |t: &Vec<u8>| hex::encode_upper(t)
        );
        println!("params={}", params);
        QueryParams::from(vec![("params", params.as_str())]).stringify()
    }

    pub fn weapi(text: &str) -> String {
        println!("text={:?}", text);
        let mut secret_key = [0u8; 16];
        OsRng.fill_bytes(&mut secret_key);
        let key: Vec<u8> = secret_key.iter().map(|i| {
            BASE62[ (i % 62) as usize ]
        }).collect();

        println!("key={}", String::from_utf8(key.clone()).unwrap());

        let params1 = Crypto::aes_encrypt(
            text,
            &*PRESET_KEY,
            cbc,
            Some(&*IV),
            |t: &Vec<u8>| base64::encode( t )
        );

        let params = Crypto::aes_encrypt(
            &params1,
            &key,
            cbc,
            Some(&*IV),
            |t: &Vec<u8>| base64::encode( t )
        );

        let enc_sec_key = Crypto::rsa_encrypt(
            std::str::from_utf8(
                &key.iter().rev().map(|n|*n)
                    .collect::<Vec<u8>>()
            ).unwrap(),
            &*RSA_PUBLIC_KEY
        );

        QueryParams::from(vec![
            ("params", params.as_str()),
            ("encSecKey", enc_sec_key.as_str())
        ]).stringify()
    }

    pub fn linuxapi(text: &str) -> String {
        let params = Crypto::aes_encrypt(
            text,
            &*LINUX_API_KEY,
            ecb,
            None,
            |t:&Vec<u8>| hex::encode(t)
        ).to_uppercase();
        println!("text={},prams={}", text, params);
        QueryParams::from(vec![
            ("eparams", params.as_str())
        ]).stringify()
    }

    pub fn aes_encrypt (
        data: &str,
        key: &Vec<u8>,
        mode: AesMode,
        iv: Option<&[u8]>,
        encode: fn(&Vec<u8>) -> String
    ) -> String {
        let cipher = match mode {
            cbc => Cipher::aes_128_cbc(),
            ecb => Cipher::aes_128_ecb(),
        };
        let cipher_text = encrypt(
            cipher,
            key,
            iv,
            data.as_bytes()
        ).unwrap();

        encode(&cipher_text)
    }

    pub fn rsa_encrypt(data: &str, key: &Vec<u8>) -> String {
        let rsa = Rsa::public_key_from_pem(key).unwrap();

        let prefix = vec![0u8; 128-data.len()];

        let data = [&prefix[..], &data.as_bytes()[..]].concat();

        let mut buf = vec![0; rsa.size() as usize];

        rsa.public_encrypt(&data, &mut buf, Padding::NONE).unwrap();

        hex::encode(buf)
    }

    pub fn hash_encrypt(data: &str, algorithm: HashType, encode: fn(DigestBytes) -> String) -> String {
        match algorithm {
            HashType::md5 => {
                encode(hash(MessageDigest::md5(), data.as_bytes()).unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Crypto;
    use crate::crypto::{
        IV, PRESET_KEY, RSA_PUBLIC_KEY, HashType, AesMode,
    };
    use urlqstring::QueryParams;

    #[test]
    fn test_aes_encrypt() {
        let msg1 = r#"{"ids":"[347230]","br":999000}"#;
        let key1 = "gLiwKFot44HYFRAy";
        let res = Crypto::aes_encrypt(
            msg1,
            &*PRESET_KEY,
            AesMode::cbc,
            Some(&*IV),
            |t: &Vec<u8>| base64::encode( t ) );
        assert_eq!(res, "pgHP1O/hr+IboRMAq6HzpHjyYwNlv1x0G4BBjd1ohdM=");

        let res2 = Crypto::aes_encrypt(
            &res,
            &key1.as_bytes().to_vec(),
            AesMode::cbc,
            Some(&*IV),
            |t: &Vec<u8>| base64::encode( t ) );
        assert_eq!(res2, "3EC4ojigTl0OgjyYtcd+97P7YKarculWrOxSgNO5clkQftvO1jOvS8aAhK6diyOb");

        let msg2 = r#"{"s":"海阔天空"}"#;
        let key2 = "05EBdrdgLjgiqaRc";
        let res = Crypto::aes_encrypt(
            msg2,
            &*PRESET_KEY,
            AesMode::cbc,
            Some(&*IV),
            |t: &Vec<u8>| base64::encode( t )
        );
        assert_eq!(res, "1CH1yTIZN/TXvOMJWH3yAe+iY8c9VfW36l3IfOm58l0=");

        let res2 = Crypto::aes_encrypt(
            &res,
            &key2.as_bytes().to_vec(),
            AesMode::cbc,
            Some(&*IV),
            |t: &Vec<u8>| base64::encode( t )
        );
        assert_eq!(res2, "uPCj4YGmXlMcix5LDAGFb0ynzwPFpFet8dZZ6ia8d2mS47OlnguVmNjGDWPJY1G3");
    }

    #[test]
    fn test_rsa_encrypt() {
        let key2 = "yARFYH44toFKwiLg";
        let res = Crypto::rsa_encrypt(key2, &*RSA_PUBLIC_KEY);
        assert_eq!(res, "5ff8bdb3ed3dd15a26e9025e9abcff0d7a3764dafbc70e33859a892584c681f1aab314b8ad1f3418650ff851bdb0685fc5136a88e059c592da104bbeaba666fbe89eb405c7b66eab4db8ee3ab13a3f98cb41b2ac9981ed4e441ed8e1870524d001ee6ebc1c09f7a945677e5b56a3e964a224c3ee75ac43fbf513f6a8bf7472ee");
    }

    #[test]
    fn test_hash_encrypt() {
        let msg = "password=uitKHY29LJ28jlFJFwoWiu1098f";
        assert_eq!(Crypto::hash_encrypt(
            msg,
            HashType::md5,
            hex::encode ), "1a72fd2483743c6b1af60041af3edd20");

        let pw = "email2158";

        assert_eq!(Crypto::hash_encrypt(
            pw,
            HashType::md5,
            hex::encode ), "afafe22f87fb761d97b8897e00e98fac");
    }

    #[test]
    fn test_weapi() {
        let text = r#"{"ids":"[\"89ADDE33C0AAE8EC14B99F6750DB954D\"]","resolution":"1080"}"#;
        let key: Vec<u8> = "IJGckGcNzgsdFNZu".as_bytes().to_vec();

        let params1 = Crypto::aes_encrypt(
            text,
            &*PRESET_KEY,
            AesMode::cbc,
            Some(&*IV),
            |t: &Vec<u8>| base64::encode( t )
        );

        let params = Crypto::aes_encrypt(
            &params1,
            &key,
            AesMode::cbc,
            Some(&*IV),
            |t: &Vec<u8>| base64::encode( t )
        );

        println!("params1={}\nparams={}", params1, params);

        let enc_sec_key = Crypto::rsa_encrypt(
            std::str::from_utf8(
                &key.iter().rev().map(|n|*n)
                    .collect::<Vec<u8>>()
            ).unwrap(),
            &*RSA_PUBLIC_KEY
        );

        let res = QueryParams::from(vec![
            ("params", params.as_str()),
            ("encSecKey", enc_sec_key.as_str())
        ]).stringify();
        println!("res={}", res);
    }

    #[test]
    fn test_linuxapi() {
        let msg = r#"{"method":"POST","url":"https://music.163.com/api/song/lyric?lv=-1&kv=-1&tv=-1","params":{"id":"347230"}}"#;
        println!("msg={}", msg);
        let res = Crypto::linuxapi(msg);
        assert_eq!(res, "eparams=A0D9583F4C5FF68DE851D2893A49DE98FAFB24399F27B4F7E74C64B6FC49A965CFA972FA5EA3D6247CD6247C8198CB873B98A81F6838B428B103E7871611EAC556D5DBE4408FD2751C0E2AD139004A718B72FE3E65ECD467E96A996D93F627A05EB0AAB74EC2E68145C014D505562560&");
    }
}
