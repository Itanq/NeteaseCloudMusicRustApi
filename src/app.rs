
use actix_web::{
    HttpRequest, HttpResponse, HttpServer,
    Responder, App, FromRequest,
    web, error, middleware, get,
};
use actix_web::http::{ Uri, };
use reqwest::header::{
    HOST, CONTENT_TYPE, USER_AGENT, REFERER,
    HeaderMap
};
use serde::Deserialize;
use listenfd::ListenFd;
use urlqstring::{
    querystring,
    querystring::QueryParamGet
};

use super::api::{
    SongInfo,
    EmailLoginInfo,
    SearchInfo,
};
use crate::api;
use crate::api::{
    CommentInfo, Identify, CellPhoneLoginInfo, MvInfo,
    TopMvInfo, TopAlbumInfo, TopList, ArtistAlbum,
    NewSong, DjInfo, Identity, PageIndex, CateId,
    PlayListDetail, ResourceType,NickName,
};
use actix_web::web::service;
use actix_http::error::PayloadError::Http2Payload;
use actix_http::http::HeaderValue;
use lazy_static::lazy_static;
use actix_http::cookie::Cookie;
use actix_web::dev::RequestHead;
use std::ops::Deref;

lazy_static!{
    static ref CONTENT_TP: HeaderValue = HeaderValue::from_static("application/json; charset=utf-8");
}

fn index_root() -> impl Responder {
    println!("index_root.....");
    HttpResponse::Ok().body("Hello World!")
}

#[get("/activate/init/profile")]
fn index_activate_init_profile( req: HttpRequest ) -> impl Responder {
    let url = "http://music.163.com/eapi/activate/initProfile";
    let cookie = req.headers().get("Cookie").unwrap();
    let value = querystring::json(req.query_string());
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "eapi",
                url,
                &value
            )
        )
}

#[get("/song/url")]
fn index_song_url( req: HttpRequest ) -> impl Responder {
    let url = "https://music.163.com/api/song/enhance/player/url";
    let query = req.query_string().replace_key("id", "ids");
    let query = query.deref();
    let br = query.get_value("br").unwrap_or("&br=999000");
    let ids = query.get_value("ids").unwrap();
    let query = query.replace_value(
        ids,
        &format!("[{}]", ids)
    );
    let query = query + br;
    let value = querystring::json(&query);
    let cookies = req.headers().get("Cookie");
    if !headers.contains_key("MUSIC_U") {

    }
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "linuxapi",
            url,
            &value
        )
    )
}

#[get("/mv/url")]
fn index_mv(info: web::Query<MvInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/song/enhance/play/mv/url";

    let value = info.into_inner();

    println!("text={}", value.to_string());

    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/top/mv")]
fn index_top_mv(info: web::Query<TopMvInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/mv/toplist";
    let value = info.into_inner();
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/artist/album")]
fn index_artist_album(web::Query(info): web::Query<ArtistAlbum>) -> impl Responder {
    let url = format!("https://music.163.com/weapi/artist/albums/{}", info.id);
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            &url,
            &info
        )
    )
}

#[get("/artist/desc")]
fn index_artist_desc(info: web::Query<Identity>) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/introduction";
    let value = info.into_inner();
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/album")]
fn index_album(info: web::Query<Identity>) -> impl Responder {
    let url = &format!("https://music.163.com/weapi/v1/album/{}", info.id);
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &"{}"
        )
    )
}

#[get("/album/detail/dynamic")]
fn index_album_detail(
    web::Query(info): web::Query<Identity>,
    header: HttpRequest
) -> impl Responder {
    let url = "https://music.163.com/api/album/detail/dynamic";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/top/album")]
fn index_top_album(info: web::Query<TopAlbumInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/album/new";
    let value = info.into_inner();
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/album/newest")]
fn index_album_newest(msg: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/discovery/newAlbum";
    println!("msg: {:?}", msg.headers().get("Cookie"));
    println!("query_string={}", msg.query_string());
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &"{}"
        )
    )
}

#[get("/album/sublist")]
fn index_album_sublist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/album/sublist";
    let value = querystring::json(req.query_string());
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/album/sub")]
fn index_album_sub(req: HttpRequest ) -> impl Responder {
    let query = req.query_string();

    let url = format!("https://music.163.com/api/album/{}", query);
}

fn index_song_comment(info: web::Query<CommentInfo>) ->impl Responder {
    let value = info.into_inner();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_SO_4_{}", value.id);
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "weapi",
            &url,
            &value
        )
    )
}

#[get("/comment/album")]
fn index_album_comment(req: HttpRequest) -> impl Responder {
    let query_string = req.query_string();
    let value = querystring::json(query_string);
    let id = query_string.get_value("id").unwrap_or("");
    println!("query_string:{}; value:{}, id:{}", query_string, value, id);
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_AL_3_{}", id);
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "weapi",
            &url,
            &value
        )
    )
}

#[get("/comment/playlist")]
fn index_playlist_comment(info: web::Query<CommentInfo>) -> impl Responder {
    let value = info.into_inner();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/A_PL_0_{}", value.id);
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "weapi",
            &url,
            &value
        )
    )
}

#[get("/comment/mv")]
fn index_mv_comment(info: web::Query<CommentInfo>) -> impl Responder {
    let value = info.into_inner();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_MV_5_{}", value.id);
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "weapi",
            &url,
            &value
        )
    )
}

#[get("/comment/dj")]
fn index_dj_comment(info: web::Query<CommentInfo>) -> impl Responder {
    let value = info.into_inner();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/A_DJ_1_{}", value.id);
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &value
            )
        )
}

#[get("/comment/video")]
fn index_video_comment(info: web::Query<CommentInfo>) -> impl Responder {
    let value = info.into_inner();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_VI_62_{}", value.id);
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "weapi",
            &url,
            &value
        )
    )
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

    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "weapi",
            &url,
            &value
        )
    )
}

#[get("/lyric")]
fn index_song_lyric(info: web::Query<Identify>) -> impl Responder {
    let url = "https://music.163.com/weapi/song/lyric?lv=-1&kv=-1&tv=-1";
    let value = info.into_inner();

    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "linuxapi",
            url,
            &value
        )
    )
}

#[get("/playlist/catlist")]
fn index_playlist_catlist() -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/catalogue";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &"{}"
        )
    )
}

#[get("/playlist/hot")]
fn index_playlist_hot() -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/hottags";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &"{}"
        )
    )
}

#[get("/playlist/detail")]
fn index_playlist_detail(web::Query(info): web::Query<PlayListDetail>, headers: HttpRequest)
    -> impl Responder
{
    let url = "https://music.163.com/weapi/v3/playlist/detail";
    let cookie = headers.headers().get("Cookie").unwrap();
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "linuxapi",
                url,
                &info
            )
        )
}

#[get("/top/list")]
fn index_top_list(info: web::Query<TopList>, headers: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v3/playlist/detail";
    let cookie = headers.headers().get("Cookie").unwrap();
    println!("Cookie: {:?}", cookie.to_str());
    let value = info.into_inner();
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "Post",
            "",
            "linuxapi",
            url,
            &value
        )
    )
}

#[get("/toplist/detail")]
fn index_toplist_detail() -> impl Responder {
    let url = "https://music.163.com/weapi/toplist/detail";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "linuxapi",
            url,
            &"{}"
        )
    )
}

#[get("/toplist/artist")]
fn index_toplist_artist() -> impl Responder {
    let url = "https://music.163.com/weapi/toplist/artist";
    let value = format!(r#"{{"type":"1","limit":"100","offset":"0","total":"true"}}"#);
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &value
        )
    )
}

#[get("/top/song")]
fn index_top_song(info: web::Query<NewSong>) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/new/songs";
    let value = info.into_inner();
    HttpResponse::Ok()
        .content_type(
            &*CONTENT_TP
        )
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/banner")]
fn index_banner(web::Query(info): web::Query<ResourceType>) -> impl Responder {
    let url = "https://music.163.com/api/v2/banner/get";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "linuxapi",
                url,
                &info
            )
        )
}

#[get("/dj/banner")]
fn index_dj_banner() -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/banner/get";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
        )
    )
}

#[get("/dj/hot")]
fn index_dj_hot(info: web::Query<DjInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/hot/v1";
    let value = format!(r#"{{"cat":"undefined","cateId":"undefined","type":"undefined","categoryId":"undefined","limit":"{}","offset":"{}"}}"#, info.limit.unwrap_or(30),info.offset.unwrap_or(0));
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/dj/recommend")]
fn index_dj_recommend() -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/recommend/v1";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &"{}"
        )
    )
}

#[get("/dj/recommend/type")]
fn index_dj_recommend_type(info: web::Query<CateId>) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/recommend";
    let value = info.into_inner();
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/dj/catelist")]
fn index_dj_category_list() -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/category/get";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &"{}"
        )
    )
}

#[get("/dj/category/excludehot")]
fn index_dj_category_exclude_hot() -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/category/excludehot";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &"{}"
        )
    )
}

#[get("/dj/category/recommend")]
fn index_dj_category_recommend() -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/home/category/recommend";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &"{}"
        )
    )
}

#[get("/dj/program/detail")]
fn index_dj_program_details(info: web::Query<Identify>) -> impl Responder {
    let url = "https://music.163.com/weapi/dj/program/detail";
    let value = info.into_inner();
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/dj/detail")]
fn index_dj_detail(info: web::Query<Identity>) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/get";
    let value = info.into_inner();
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/dj/toplist/pay")]
fn index_dj_pay_gift(info: web::Query<DjInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/home/paygift/list?_nmclfl=1";
    let value = info.into_inner();
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/dj/sub")]
fn index_dj_sub(info: web::Query<Identity>) -> impl Responder {
    let url = format!("https://music.163.com/weapi/djradio/{}",
        if let Some(t) = info.other {
            if t == 1 {
                "sub"
            } else {
                "unsub"
            }
        } else {
            "unsub"
        }
    );
    let value = info.into_inner();
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            &url,
            &value
        )
    )
}

#[get("/dj/sublist")]
fn index_dj_sub_list(info: web::Query<TopMvInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/get/subed";
    let value = info.into_inner();
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/dj/today/perfered")]
fn index_dj_today_perfered(info: web::Query<PageIndex>) -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/home/today/perfered";
    let value = info.into_inner();
    HttpResponse::Ok().body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

fn index_login_with_email(info: web::Query<EmailLoginInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/login";

    let value = info.into_inner();

    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/login/cellphone")]
fn index_login_with_cellphone(info: web::Query<CellPhoneLoginInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/login/cellphone";

    let value = info.into_inner();

    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "pc",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/login/refresh")]
fn index_login_refresh() -> impl Responder {
    let url = "https://music.163.com/weapi/login/token/refresh";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "pc",
            "weapi",
            url,
            &"{}"
        )
    )
}

#[get("/recommend/songs")]
fn index_recommend_songs() -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/recommend/songs";
    let value = format!(r#"{{"limit":"{}","offset":"{}","total":"{}"}}"#,
        20, 0, true
        );
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

fn index_search(info: web::Query<SearchInfo>) -> impl Responder {
    let url = "https://music.163.com/weapi/search/get";

    let value = info.into_inner();

    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

#[get("/search/hot/detail")]
fn index_search_hot_detail() -> impl Responder {
    let url = "https://music.163.com/weapi/hotsearchlist/get";
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

fn index_search_hot() -> impl Responder {
    let url = "https://music.163.com/weapi/search/hot";
    let value = r#"{{"type":1111}}"#;
    HttpResponse::Ok()
        .content_type(&*CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &value
        )
    )
}

pub fn start_server() {
    dbg!("start_server...");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/search/hot")
                    .route(web::get().to(index_search_hot))
            )
            .service(index_activate_init_profile)
            .service(index_search_hot_detail)
            .service(index_artist_album)
            .service(index_artist_desc)
            .service(index_album)
            .service(index_album_detail)
            .service(index_top_album)
            .service(index_album_sublist)
            .service(index_album_newest)
            .service(index_dj_recommend_type)
            .service(index_dj_recommend)
            .service(index_dj_sub)
            .service(index_dj_today_perfered)
            .service(index_dj_detail)
            .service(index_dj_program_details)
            .service(index_dj_pay_gift)
            .service(index_dj_category_recommend)
            .service(index_dj_category_exclude_hot)
            .service(index_dj_category_list)
            .service(index_dj_hot)
            .service(index_banner)
            .service(index_dj_banner)
            .service(index_album_comment)
            .service(index_dj_comment)
            .service(index_mv_comment)
            .service(index_video_comment)
            .service(index_playlist_comment)
            .service(index_playlist_catlist)
            .service(index_playlist_detail)
            .service(index_playlist_hot)
            .service(index_top_list)
            .service(index_toplist_detail)
            .service(index_toplist_artist)
            .service(index_top_song)
            .service(index_hot_comment)
            .service(index_song_lyric)
            .service(index_login_with_cellphone)
            .service(index_login_refresh)
            .service(index_mv)
            .service(index_top_mv)
            .service(index_recommend_songs)
            .service(index_song_url)
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
        server.bind("127.0.0.1:8000").unwrap()
    };

    server.run().unwrap();
}