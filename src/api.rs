use actix_web::{
    App, HttpRequest, FromRequest, HttpResponse,
    HttpServer, Responder, web, error,
    cookie::Cookie,
};
use actix_web::http::{ Uri, };
use reqwest::header::{HOST, CONTENT_TYPE, USER_AGENT, REFERER, HeaderMap, ToStrError, CONTENT_ENCODING, COOKIE};
use reqwest::blocking::{
    ClientBuilder,
    Client,
};
use serde::Deserialize;
use urlqstring::QueryParams;

use super::crypto::Crypto;
use actix_web::error::UrlencodedError::ContentType;
use crate::crypto::HashType;
use base64::CharacterSet::Crypt;
use rand::{ thread_rng, Rng };
use std::cell::Ref;
use actix_web::http::header::Date;
use std::time::SystemTime;

pub const banner_type: [&str; 4] = [
    "pc", "android", "iphone", "ipad"
];

pub const resource_type: [&str; 7] = [
    "R_SO_4_",  //  歌曲
    "R_MV_5_",  //  MV
    "A_PL_0_",  //  歌单
    "R_AL_3_",  //  专辑
    "A_DJ_1_",  //  电台,
    "R_VI_62_", //  视频
    "A_EV_2_"   //  动态
];

pub const operator: [&str; 3] = [
    "delete", "add", "reply"
];

const linux_user_agent: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.90 Safari/537.36";

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

pub const topList: [&str; 37] = [
    "3779629", //云音乐新歌榜
    "3778678", //云音乐热歌榜
    "2884035", //云音乐原创榜
    "19723756", //云音乐飙升榜
    "10520166", //云音乐电音榜
    "180106", //UK排行榜周榜
    "60198", //美国Billboard周榜
    "21845217", //KTV嗨榜
    "11641012", //iTunes榜
    "120001", //Hit FM Top榜
    "60131", //日本Oricon周榜
    "3733003", //韩国Melon排行榜周榜
    "60255", //韩国Mnet排行榜周榜
    "46772709", //韩国Melon原声周榜
    "112504", //中国TOP排行榜(港台榜)
    "64016", //中国TOP排行榜(内地榜)
    "10169002", //香港电台中文歌曲龙虎榜
    "4395559", //华语金曲榜
    "1899724", //中国嘻哈榜
    "27135204", //法国 NRJ EuroHot 30周榜
    "112463", //台湾Hito排行榜
    "3812895", //Beatport全球电子舞曲榜
    "71385702", //云音乐ACG音乐榜
    "991319590", //云音乐说唱榜,
    "71384707", //云音乐古典音乐榜
    "1978921795", //云音乐电音榜
    "2250011882", //抖音排行榜
    "2617766278", //新声榜
    "745956260", //云音乐韩语榜
    "2023401535", //英国Q杂志中文版周榜
    "2006508653", //电竞音乐榜
    "2809513713", //云音乐欧美热歌榜
    "2809577409", //云音乐欧美新歌榜
    "2847251561", //说唱TOP榜
    "3001835560", //云音乐ACG动画榜
    "3001795926", //云音乐ACG游戏榜
    "3001890046", //云音乐ACG VOCALOID榜
];

pub struct RequestParams<'a>
{
    pub ua: &'a str,
    pub crypto: &'a str,
    pub cookies: Option<Ref<'a, Vec<Cookie<'a>>>>,
    pub url: Option<&'a str>,
}

trait CookiesExtensionMethods {
    fn get_cookies_value(&self, key: &str) -> &str;
    fn get_cookies_value_or(&self, key: &str) -> Option<&str>;
}

impl<'a> CookiesExtensionMethods for Vec<Cookie<'a>> {
    fn get_cookies_value(&self, key: &str) -> &str {
        self.iter().find_map(|val|{
            if val.name() == key {
                return Some(val.value());
            } else {
                return None;
            }
        }).unwrap_or("")
    }

    fn get_cookies_value_or(&self, key: &str) -> Option<&str> {
        let val = self.get_cookies_value(key);
        if val.is_empty() {
            return None;
        } else {
            return Some(val);
        }
    }
}

pub fn create_request(
    method: &str,
    url: &str,
    value: &QueryParams,
    params: &RequestParams
    ) -> serde_json::Value {

    let cookies = params.cookies.as_ref().unwrap();
    let crypto = params.crypto;
    let ua = params.ua;

    let mut headers = HeaderMap::new();

    if method.to_uppercase() ==  "POST" {
        headers.insert(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse().unwrap());
    }
    if url.contains("music.163.com") {
        headers.insert(
            REFERER,
            "https://music.163.com".parse().unwrap());
    }
    headers.insert(
        CONTENT_ENCODING,
        "utf-8".parse().unwrap()
    );
    headers.insert(
        USER_AGENT,
        choose_user_agent(ua).parse().unwrap()
    );

    let body = match crypto {
        "eapi" => {
            let csrf_token = cookies.get_cookies_value("__csrf");
            let osver = cookies.get_cookies_value("osver");
            let deviceId = cookies.get_cookies_value("deviceId");
            let versioncode = cookies.get_cookies_value_or("versioncode")
                .unwrap_or("140");
            let mobilename = cookies.get_cookies_value("mobilename");
            let _date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis().to_string();
            let buildver = cookies.get_cookies_value_or("buildver").unwrap_or(&_date);
            let resolution = cookies.get_cookies_value_or("resolution")
                .unwrap_or("1920x1080");
            let os = cookies.get_cookies_value_or("os").unwrap_or("android");
            let channel = cookies.get_cookies_value("channel");
            let rng = thread_rng().gen_range(0, 1000);
            let id = format!("{:04}", rng);
            let requestId = &format!("{}_{}",
                SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap().as_millis(), id
            );

            let value = value.clone();
            let value = value.add_query_string("osver", osver)
                .add_query_string("deviceId", deviceId)
                .add_query_string("versioncode", versioncode)
                .add_query_string("mobilename", mobilename)
                .add_query_string("buildver", buildver)
                .add_query_string("resolution", resolution)
                .add_query_string("os", os)
                .add_query_string("channel", channel)
                .add_query_string("requestId", requestId);
            let value = if let Some(val) = cookies.get_cookies_value_or("MUSIC_U") {
                value.add_query_string("MUSIC_U", val)
            } else if let Some(val) = cookies.get_cookies_value_or("MUSIC_A") {
                value.add_query_string("MUSIC_A", val)
            } else {
                value
            };

            Crypto::eapi(params.url.unwrap(), &value.json())
        },
        "weapi" => {
            let csrf_token = cookies.iter().find_map(|val|{
                if val.name() == "__csrf" {
                    return Some(val.value());
                } else {
                    return None;
                }
            }).unwrap_or("");

            println!("csrf_token: {}", csrf_token);

            let value = if !csrf_token.is_empty() {
                value.clone().add_query_string("csrf_token", csrf_token)
                    .json()
            } else {
                value.json()
            };
            Crypto::weapi(&value)
        },

        "linuxapi" => {
            headers.insert(
                USER_AGENT,
                linux_user_agent.parse().unwrap()
            );
            let data = format!(
                r#"{{"method":"{}","url":"{}","params":{}}}"#,
                method,
                url.replace("weapi", "api"),
                &value.json() );
            println!("data={}", data);
            Crypto::linuxapi(&data)
        },
        _ => Crypto::weapi(&value.json()),
    };

    let url = match crypto {
        "linuxapi" => {
           "https://music.163.com/api/linux/forward"
        },
        _ => url,
    };

    println!("body={}", body);

    let client = ClientBuilder::new()
        .default_headers(headers)
        .cookie_store(true)
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

    unsafe {
        user_agent_list.get_unchecked(index)
    }
}
