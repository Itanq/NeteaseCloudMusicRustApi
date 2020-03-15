
use actix_web::{HttpResponse, HttpServer, HttpRequest, Responder, HttpMessage, get, web, App};
use urlqstring::QueryParams;
use percent_encoding::percent_decode_str;
use std::borrow::Cow;

use crate::request::generate_response;

#[get("/song/url")]
pub(crate) async fn index_song_url(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/song/enhance/player/url";
    let cookies = req.cookies().unwrap().iter().fold(String::from(""),|acc,val| {
        val.to_string() + &acc
    });
    println!("cookies={}", cookies);
    let query_string = QueryParams::from(req.query_string());

    let ids = "[".to_owned() + query_string.value("id").unwrap() + "]";
    let query_params = json_object!({
        "ids": ids.as_str(),
        "br": query_string.value("br").unwrap_or("999000")
    });

    let request_params = json_object!({
        "crypto": "linuxapi",
        "cookie": &cookies,
        "proxy": ""
    });

    generate_response(
        url,
        "POST",
        query_params,
        request_params
    ).await
}

#[get("/search")]
pub(crate) async fn index_search(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/search/get";
    let cookies = req.cookies().unwrap().iter().fold(String::from(""),|acc,val| {
         val.to_string() + &acc
    });
    println!("cookies={}", cookies);


    let qs = percent_decode_str(req.query_string())
        .decode_utf8().unwrap_or(Cow::Borrowed(&""));

    let query_string = QueryParams::from(qs.as_ref());

    let query_params = json_object!({
        "s": query_string.value("keywords").unwrap_or(""),
        "type": query_string.value("type").unwrap_or("1"),
        "limit": query_string.value("limit").unwrap_or("30"),
        "offset": query_string.value("offset").unwrap_or("0")
    });

    let request_params = json_object!({
        "crypto": "weapi",
        "cookie": &cookies,
        "proxy": ""
    });

    generate_response(
        url,
        "POST",
        query_params,
        request_params
    ).await
}
