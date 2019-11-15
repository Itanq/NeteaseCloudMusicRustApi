
use actix_web::{ App, HttpRequest, FromRequest, HttpResponse, HttpServer, Responder, web, error };
use actix_web::http::{ Uri, };
use reqwest::header::{
    HOST, CONTENT_TYPE, USER_AGENT, REFERER,
    HeaderMap
};
use serde::Deserialize;
use futures::executor::block_on;

use super::crypto::Crypto;


fn index_root() -> impl Responder {
    println!("index_root.....");
    HttpResponse::Ok().body("Hello World!")
}

fn search_song(value: &SongInfo) -> String {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (Linux; Android 5.1.1; Nexus 6 Build/LYZ28E) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36".parse().unwrap());
    headers.insert(REFERER, "https://music.163.com".parse().unwrap());

    let client = reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let body = Crypto::weapi(&value.to_string());

    let res = client.post("https://music.163.com/weapi/song/enhance/player/url")
        .body(body)
        .send().unwrap()
        .text().unwrap();

    println!("{:?}", res);

    res
}

#[derive(Deserialize)]
pub struct SongInfo {
    id: String,
    br: u32,
}

impl ToString for SongInfo {

    fn to_string(&self) -> String {
        format!(r#"{{"ids":"[{}]","br":{}}}"#, self.id, self.br)
    }
}

fn index_song(info: web::Query<SongInfo>) -> impl Responder {

    let value = info.into_inner();

    let res = search_song(&value);

    HttpResponse::Ok().body(res)
}


pub fn start_server() {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/song/url")
                    .data(
                        web::Query::<SongInfo>::configure(|cfg| {
                            cfg.error_handler(|err, _req| {
                                println!("Internal Error!! errInfo={}", err);
                                error::InternalError::from_response(
                                    err,
                                    HttpResponse::Conflict().finish(),
                                )
                                    .into()
                            })
                        }),
                    ).route(web::get().to(index_song))
            )
            .route("/", web::get().to(index_root))
    })
    .bind("localhost:8000").unwrap()
    .run().unwrap();
}