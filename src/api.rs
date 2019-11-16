use actix_web::{ App, HttpRequest, FromRequest, HttpResponse, HttpServer, Responder, web, error };
use actix_web::http::{ Uri, };
use reqwest::header::{
    HOST, CONTENT_TYPE, USER_AGENT, REFERER,
    HeaderMap
};
use reqwest::blocking::{
    ClientBuilder,
    Client,
};
use serde::Deserialize;

use super::crypto::Crypto;
use actix_web::error::UrlencodedError::ContentType;
use crate::crypto::HashType;
use base64::CharacterSet::Crypt;
use rand::rngs::OsRng;
use rand::Rng;

const user_agent_list: [&str; 14] = [
    "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0 Mobile/13B143 Safari/601.1",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0 Mobile/13B143 Safari/601.1",
    "Mozilla/5.0 (Linux; Android 5.0; SM-G900P Build/LRX21T) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 5.1.1; Nexus 6 Build/LYZ28E) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 10_3_2 like Mac OS X) AppleWebKit/603.2.4 (KHTML, like Gecko) Mobile/14F89;GameHelper",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 10_0 like Mac OS X) AppleWebKit/602.1.38 (KHTML, like Gecko) Version/10.0 Mobile/14A300 Safari/602.1",
    "Mozilla/5.0 (iPad; CPU OS 10_0 like Mac OS X) AppleWebKit/602.1.38 (KHTML, like Gecko) Version/10.0 Mobile/14A300 Safari/602.1",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.12; rv:46.0) Gecko/20100101 Firefox/46.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_5) AppleWebKit/603.2.4 (KHTML, like Gecko) Version/10.1.1 Safari/603.2.4",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:46.0) Gecko/20100101 Firefox/46.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/13.1058",
];

pub fn create_request<T: ToString>(method: &str, ua: &str, crypto: &str, url: &str, value: &T) -> serde_json::Value {

    let mut headers = HeaderMap::new();

    if method.to_uppercase() ==  "POST" {
        headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
    }
    if url.contains("music.163.com") {
        headers.insert(REFERER, "https://music.163.com".parse().unwrap());
    }
    headers.insert(USER_AGENT, choose_user_agent(ua).parse().unwrap());

    let body = match crypto {
        "weapi" => Crypto::weapi(&value.to_string()),
        "linuxapi" => {
            let data = format!(r#"{{"method":"{}","url":"{}","params":{}}}"#, method, url.replace("weapi", "api"), value.to_string());
            println!("data={}", data);
            Crypto::linuxapi(&data)
        },
        _ => Crypto::weapi(&value.to_string()),
    };

    let url = match crypto {
        "linuxapi" => {
            headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.90 Safari/537.36".parse().unwrap());
            "https://music.163.com/api/linux/forward"
        },
        _ => url,
    };

    println!("{},body={}",crypto, body);

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    client.post(url)
        .body(body)
        .send().unwrap()
        .json().unwrap()
}

fn choose_user_agent(ua: &str) -> &str {
    let index = if ua == "mobile" {
        rand::thread_rng().gen_range(0,7)
    } else if ua == "pc" {
        rand::thread_rng().gen_range(0, 5) + 8
    } else {
        rand::thread_rng().gen_range(0, user_agent_list.len())
    };

    println!("userAgent={}", user_agent_list[index]);

    unsafe {
        user_agent_list.get_unchecked(index)
    }
}


#[derive(Deserialize)]
pub struct SongInfo {
    id: String,
    br: Option<u32>,
}

impl ToString for SongInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"ids":"[{}]","br":{}}}"#, self.id, self.br.unwrap_or(999000))
    }
}

#[derive(Deserialize)]
pub struct MvInfo {
    id: String,
    r: Option<u32>,
}

#[derive(Deserialize)]
pub struct TopMvInfo {
    limit: Option<u32>,
    offset: Option<u32>,
    total: Option<bool>,
}

impl ToString for TopMvInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"limit":"{}","offset":"{}","total":"{}"}}"#,
            self.limit.unwrap_or(30),
            self.offset.unwrap_or(0),
            self.total.unwrap_or(true)
        )
    }
}

impl ToString for MvInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"id":"{}","r":{}}}"#, self.id, self.r.unwrap_or(1080))
    }
}

#[derive(Deserialize)]
pub struct CommentInfo {
    pub id: String,
    limit: Option<u32>,
    offset: Option<u32>,
    before: Option<u32>,
}

impl ToString for CommentInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"rid":"{}","limit":"{}","offset":"{}"}}"#,
            self.id,
            self.limit.unwrap_or(20),
            self.offset.unwrap_or(0)
        )
    }
}

#[derive(Deserialize)]
pub struct EmailLoginInfo {
    email: String,
    password: String,
    rememberLogin: Option<bool>,
}

impl ToString for EmailLoginInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"username":"{}","password":"{}","rememberLogin":"{}"}}"#,
                self.email,
                Crypto::hash_encrypt(&self.password, HashType::md5, hex::encode),
                self.rememberLogin.unwrap_or(true)
        )
    }
}

#[derive(Deserialize)]
pub struct CellPhoneLoginInfo {
    phone: String,
    password: String,
    countryCode: Option<u16>,
    rememberLogin: Option<bool>,
}

impl ToString for CellPhoneLoginInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"phone":"{}","password":"{}","rememberLogin":"{}"}}"#,
            self.phone,
            Crypto::hash_encrypt(&self.password, HashType::md5, hex::encode),
            self.rememberLogin.unwrap_or(true)
        )
    }
}

#[derive(Deserialize)]
pub struct SearchInfo {
    keywords: String,
    types: Option<u32>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl ToString for SearchInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"s":"{}","type":"{}","limit":"{}","offset":"{}"}}"#,
            self.keywords,
            self.types.unwrap_or(1),
            self.limit.unwrap_or(30),
            self.offset.unwrap_or(0)
        )
    }
}

#[derive(Deserialize)]
pub struct Identify {
    id: u32
}

impl ToString for Identify {
    fn to_string(&self) -> String {
        format!(r#"{{"id":"{}"}}"#, self.id)
    }
}
