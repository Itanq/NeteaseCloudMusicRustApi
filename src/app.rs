
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

async fn search_song(value: &SongInfo) -> Result<String, reqwest::Error>  {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (Linux; Android 5.1.1; Nexus 6 Build/LYZ28E) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36".parse().unwrap());
    headers.insert(REFERER, "https://music.163.com".parse().unwrap());

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let (params, encSecKey) = Crypto::weapi(&value.to_string());
//    let body = format!("params={}encSecKey={}", params, encSecKey);

    let body = "W%2BS8Ho5Tlwj1Kp3vkC6b%2F1aZTP2bDmVZo%2FNJdtVmrs0Pd6yMEKJ9vo4hCLSRoFKmlR1JDk5lXfHjEN0XChQr60CFzf9OEeK7S%2FBIOwbg9w%3D&encSecKey=a15a23acd13c3112b850d70c9d3d6dceb5bddde7e329cfc9443ff03dd761e151f088e8cbbdda199021110e9084ae784f4a79d0dae84acc5da500743c9abb8b8002b3d7ed7680079f3ff132aa1b0ebdf91e94e8b513fcdea7715b0ee87d31374b50d9ecf0d1c5581c09ebed7b7a3052fc925e661d3304e96d090fcab37b99c517";

    println!("body={}", body);
//    let res = "hello".to_string();

    let res = client.post("https://music.163.com/weapi/song/enhance/player/url")
        .json(&serde_json::json!({
            "params": params,
            "encSecKey": encSecKey
        }))
        .send().await?
        .text().await?;

    println!("resssssssssss = {:?}", res);

    Ok(res)
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

    println!("id={}, br={}", info.id, info.br);

    let value = info.into_inner();

    let res = block_on(search_song(&value))
        .unwrap_or("hello world!".to_string());

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
                                println!("Internal Error!!");
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
            //.route("/song/url", web::get().to(index_song))
    })
    .bind("localhost:8000").unwrap()
    .run().unwrap();
}