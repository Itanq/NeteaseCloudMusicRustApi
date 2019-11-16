
use actix_web::{
    App, HttpRequest, FromRequest, HttpResponse, HttpServer, Responder,
    web, error, middleware, get,
};
use actix_web::http::{ Uri, };
use reqwest::header::{
    HOST, CONTENT_TYPE, USER_AGENT, REFERER,
    HeaderMap
};
use serde::Deserialize;
use listenfd::ListenFd;

use super::api::{
    SongInfo,
    EmailLoginInfo,
    SearchInfo,
};
use crate::api;
use crate::api::{CommentInfo, Identify, CellPhoneLoginInfo, MvInfo, TopMvInfo};
use actix_web::web::service;


fn index_root() -> impl Responder {
    println!("index_root.....");
    HttpResponse::Ok().body("Hello World!")
}

fn index_song(info: web::Query<SongInfo>) -> impl Responder {

    let url = "https://music.163.com/weapi/song/enhance/player/url";

    let value = info.into_inner();

    HttpResponse::Ok().json(api::create_request("POST", "", "weapi", url, &value))
}

#[get("/mv/url")]
fn index_mv(info: web::Query<MvInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/song/enhance/play/mv/url";

    let value = info.into_inner();

    println!("text={}", value.to_string());

    HttpResponse::Ok().json(api::create_request("POST","", "weapi", url, &value))
}

#[get("/top/mv")]
fn index_top_mv(info: web::Query<TopMvInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/mv/toplist";

    let value = info.into_inner();

    HttpResponse::Ok().json(api::create_request("POST", "", "weapi", url, &value))
}


fn index_song_comment(info: web::Query<CommentInfo>) ->impl Responder {
    let value = info.into_inner();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_SO_4_{}", value.id);
    HttpResponse::Ok().json(api::create_request("POST", "", "weapi", &url, &value))
}

#[get("/comment/album")]
fn index_album_comment(info: web::Query<CommentInfo>) -> impl Responder {
    let value = info.into_inner();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_AL_3_{}", value.id);
    HttpResponse::Ok().json( api::create_request("POST", "", "weapi", &url, &value))
}

#[get("/comment/playlist")]
fn index_playlist_comment(info: web::Query<CommentInfo>) -> impl Responder {
    let value = info.into_inner();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/A_PL_0_{}", value.id);
    HttpResponse::Ok().json(api::create_request("POST", "", "weapi", &url, &value))
}

#[get("/comment/mv")]
fn index_mv_comment(info: web::Query<CommentInfo>) -> impl Responder {
    let value = info.into_inner();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_MV_5_{}", value.id);
    HttpResponse::Ok().json(api::create_request("POST", "", "weapi", &url, &value))
}

#[get("/comment/dj")]
fn index_dj_comment(info: web::Query<CommentInfo>) -> impl Responder {
    let value = info.into_inner();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/A_DJ_1_{}", value.id);
    HttpResponse::Ok().json(api::create_request("POST", "", "weapi", &url, &value))
}
#[get("/comment/video")]
fn index_video_comment(info: web::Query<CommentInfo>) -> impl Responder {
    let value = info.into_inner();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_VI_62_{}", value.id);
    HttpResponse::Ok().json(api::create_request("POST", "", "weapi", &url, &value))
}

#[get("/comment/{types}/hot")]
fn index_hot_comment(info: web::Query<CommentInfo>, types: web::Path<String>) -> impl Responder {
    let value = info.into_inner();
    let music = String::from("music");
    let mv = String::from("mv");
    let album = String::from("album");
    let dj = String::from("dj");
    let video = String::from("video");
    let comment_type = match types.into_inner() {
        music   => "R_SO_4_",
        mv      => "R_MV_5_",
        playlist=> "A_PL_0_",
        album   => "R_AL_3_",
        dj      => "A_DJ_1_",
        video   => "R_VI_62_",
        _ => "R_SO_4_",
    };
    let url = format!("https://music.163.com/weapi/v1/resource/hotcomments/{}{}", comment_type, value.id);

    HttpResponse::Ok().json(api::create_request("POST", "", "weapi", &url, &value))
}

#[get("/lyric")]
fn index_song_lyric(info: web::Query<Identify>) -> impl Responder {
    let url = "https://music.163.com/weapi/song/lyric?lv=-1&kv=-1&tv=-1";
    let value = info.into_inner();

    HttpResponse::Ok().json(api::create_request("POST", "", "linuxapi", url, &value))
}


fn index_login_with_email(info: web::Query<EmailLoginInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/login";

    let value = info.into_inner();

    HttpResponse::Ok().body(api::create_request("POST", "", "weapi", url, &value))
}

#[get("/login/cellphone")]
fn index_login_with_cellphone(info: web::Query<CellPhoneLoginInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/login";

    let value = info.into_inner();

    HttpResponse::Ok().json(api::create_request("POST", "pc", "weapi", url, &value))
}

fn index_search(info: web::Query<SearchInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/search/get";

    let value = info.into_inner();

    HttpResponse::Ok().json(api::create_request("POST", "", "weapi", url, &value))
}

fn index_search_hot() -> impl Responder {
    let url = "https://music.163.com/weapi/search/hot";
    let value = r#"{{"type":1111}}"#;
    HttpResponse::Ok().json( api::create_request("POST", "", "weapi", url, &value))
}


pub fn start_server() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/search/hot")
                    .route(web::get().to(index_search_hot))
            )
            .service(index_album_comment)
            .service(index_dj_comment)
            .service(index_mv_comment)
            .service(index_video_comment)
            .service(index_playlist_comment)
            .service(index_hot_comment)
            .service(index_song_lyric)
            .service(index_login_with_cellphone)
            .service(index_mv)
            .service(index_top_mv)
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
            .service(
                web::resource("/comment/music")
                    .data(
                        web::Query::<CommentInfo>::configure(|cfg| {
                            cfg.error_handler(|err, _req| {
                                println!("Internal Error: {}", err);
                                error::InternalError::from_response(
                                    err,
                                    HttpResponse::Conflict().finish(),
                                ).into()
                            })
                        })
                    ).route(web::get().to(index_song_comment))
            )
            .service(
                web::resource("/login")
                    .data(
                        web::Query::<EmailLoginInfo>::configure(|cfg| {
                            cfg.error_handler(|err, _req| {
                                println!("Internal Error: {}", err);
                                error::InternalError::from_response(
                                    err,
                                    HttpResponse::Conflict().finish(),
                                ).into()
                            })
                        })
                    ).route(web::get().to(index_login_with_email))
            )
            .service(
                web::resource("/search")
                    .data(
                        web::Query::<SearchInfo>::configure(|cfg| {
                            cfg.error_handler(|err, _req| {
                                println!("Internal Error: {}", err);
                                error::InternalError::from_response(
                                    err,
                                    HttpResponse::Conflict().finish(),
                                ).into()
                            })
                        })
                    ).route(web::get().to(index_search))
            )
            .route("/", web::get().to(index_root))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("localhost:8000").unwrap()
    };

    server.run().unwrap();
}