
use actix_web::{HttpRequest, HttpResponse, HttpServer, Responder, App, FromRequest, web, error, middleware, get, HttpMessage};
use actix_web::http::{ Uri, };
use reqwest::header::{
    HOST, CONTENT_TYPE, USER_AGENT, REFERER,
    HeaderMap
};
use serde::Deserialize;
use listenfd::ListenFd;
use urlqstring::QueryParams;
use crate::{
    api,
    api::{
        banner_type, operator, resource_type, topList,
        ToStringExtensionMethods,
        ToHashMap,
    }
};
use crate::crypto::{
    Crypto, HashType
};
use actix_web::web::{service, to};
use actix_http::error::PayloadError::Http2Payload;
use actix_http::http::HeaderValue;
use lazy_static::lazy_static;
use actix_http::cookie::Cookie;
use actix_web::dev::RequestHead;
use std::ops::Deref;
use actix_web::error::UrlencodedError::ContentType;
use base64::CharacterSet::Crypt;
use crate::api::RequestParams;

const CONTENT_TP: &'static str = "application/json; charset=utf-8";

fn index_root() -> impl Responder {
    println!("index_root.....");
    HttpResponse::Ok().body("Hello World!")
}

#[get("/activate/init/profile")]
fn index_activate_init_profile( req: HttpRequest ) -> impl Responder {
    let url = "http://music.163.com/eapi/activate/initProfile";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "eapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/album/detail/dynamic")]
fn index_album_detail_dynamic(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/album/detail/dynamic";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/album/newest")]
fn index_album_newest(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/discovery/newAlbum";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/album/sub")]
fn index_album_sub(req: HttpRequest ) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/api/album/{}", id);
    let info = QueryParams::from(query);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/album/sublist")]
fn index_album_sublist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/album/sublist";
    let query = QueryParams::from(req.query_string());

    let limit = query.value("limit").unwrap_or("25");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;

    let t = &format!("limit={}&offset={}&total={}", limit, offset, total);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/album")]
fn index_album(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap_or("32311");
    let url = &format!("https://music.163.com/weapi/v1/album/{}", id);
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/artist/album")]
fn index_artist_album(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap_or("6452");
    let url = format!("https://music.163.com/weapi/artist/albums/{}",id);

    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;

    let value = format!("limit={}&offset={}&total={}", limit,offset,total);
    let info = QueryParams::from(&value);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/artist/desc")]
fn index_artist_desc(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/introduction";
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap_or("6452");
    let t = &format!("id={}", id);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/artist/list")]
fn index_artist_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/list";
    let query = QueryParams::from(req.query_string());

    let categoryCode = query.value("cat").unwrap_or("1001");
    let offset = query.value("offset").unwrap_or("0");
    let limit = query.value("limit").unwrap_or("30");
    let total = true;
    let initial = query.value("initial").unwrap_or("undefined");
    let t = &format!("categoryCode={}&initial={}&offset={}&limit={}&total={}",
            categoryCode, initial, offset, limit, total
        );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/artist/mv")]
fn index_artist_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/mvs";
    let query =  QueryParams::from(req.query_string());

    let artistId = query.value("id").unwrap_or("6452");
    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;
    let t = &format!("artistId={}&limit={}&offset={}&total={}",
            artistId, limit, offset, total
        );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/artist/sub")]
fn index_artist_sub(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let artistId = query.value("id").unwrap_or("6452");
    let t = query.value("t").unwrap_or("0");
    let sub = if t.parse::<i32>().unwrap() == 1 {
        "sub"
    } else {
        "unsub"
    };
    let url = format!("https://music.163.com/weapi/artist/{}", sub);

    let t = &format!("artistId={}&artistIds=[{}]", artistId, artistId);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/artist/sublist")]
fn index_artist_sublist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/sublist";
    let query = QueryParams::from(req.query_string());

    let limit = query.value("limit").unwrap_or("25");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;

    let t = &format!("limit={}&offset={}&total={}", limit, offset, total);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/artist/top/song")]
fn index_artist_top_song(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/artist/top/song";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/artists")]
fn index_artists(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap_or("6452");
    let url = format!("https://music.163.com/weapi/v1/artist/{}", id);
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/banner")]
fn index_banner(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/v2/banner/get";
    let id = QueryParams::from(req.query_string()).value("type").unwrap_or("0")
        .parse::<usize>().unwrap_or(0);
    let _type = banner_type[ if id > 3 { 0 } else { id } ];
    let t = &format!("clientType={}",_type);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "linuxapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/batch")]
fn index_batch(req: HttpRequest) -> impl Responder {
   let url = "http://music.163.com/eapi/batch";

}

#[get("/captcha/register")]
fn index_captcha_register(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/register/cellphone";
    let query = QueryParams::from(req.query_string());

    let captcha = query.value("captcha").unwrap();
    let phone = query.value("phone").unwrap();
    let password = query.value("password").unwrap();
    let pw = Crypto::hash_encrypt(password, HashType::md5, hex::encode);
    let nickname = query.value("nickname").unwrap();
    let t = &format!("captcha={}&phone={}&password={}&nickname={}",
            captcha, phone, pw, nickname
        );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/captcha/sent")]
fn index_captcha_sent(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/sms/captcha/sent";
    let query = QueryParams::from(req.query_string());

    let ctcode = query.value("ctcode").unwrap_or("86");
    let cellphone = query.value("phone").unwrap();

    let t = &format!("ctcode={}&cellphone={}", ctcode, cellphone);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/captcha/verify")]
fn index_captcha_verify(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/sms/captcha/verify";
    let query = QueryParams::from(req.query_string());

    let ctcode = query.value("ctcode").unwrap_or("86");
    let cellphone = query.value("phone").unwrap();
    let captcha = query.value("captcha").unwrap();
    let t = &format!("ctcode={}&cellphone={}&captcha={}",
            ctcode, cellphone, captcha
        );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/cellphone/existence/check")]
fn index_cellphone_existence_check(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/eapi/cellphone/existence/check";
    let query = QueryParams::from(req.query_string());

    let countrycode = query.value("countrycode").unwrap_or("86");
    let cellphone = query.value("phone").unwrap();

    let t = &format!("countrycode={}&cellphone={}", countrycode, cellphone);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "eapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/check/music")]
fn index_check_music(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/enhance/player/url";
    let query = QueryParams::from(req.query_string());

    let ids = query.value("id").unwrap();
    let br = query.value("br").unwrap_or("999000");

    let t = &format!("ids={}&br={}", ids, br);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/comment")]
fn index_comment(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = operator[ query.value("t").unwrap().parse::<usize>().unwrap_or(0) ];
    let url = format!("https://music.163.com/weapi/resource/comments/{}", t);

    let rtype = resource_type[ query.value("type").unwrap().parse::<usize>().unwrap_or(0) ];
    let threadId:String = if rtype == "A_EV_2_" {
        String::from(query.value("threadId").unwrap())
    } else {
        rtype.to_owned() + query.value("id").unwrap()
    };
    let mut res = threadId;
    if t == "add" {
        res.push_str(
            &format!("&content={}",
                     query.value("content").unwrap()
            )
        );
    } else if t == "delete" {
        res.push_str(
            &format!("&commentId={}",
                query.value("commentId").unwrap()
            )
        );
    } else if t == "reply" {
        res.push_str(
            &format!("&commentId={}&content={}",
                query.value("commentId").unwrap(),
                query.value("content").unwrap()
            )
        );
    }

    let info = QueryParams::from(&res);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/comment/album")]
fn index_comment_album(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_AL_3_{}", id);

    let t = &format!("rid={}", id);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/comment/dj")]
fn index_comment_dj(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/A_DJ_1_{}", id);
    let info = QueryParams::from(query);
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/comment/event")]
fn index_comment_event(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("threadId").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/{}", id);

    let info = QueryParams::from(query);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/comment/hot")]
fn index_comment_hot(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let _type = resource_type[query.value("type").unwrap().parse::<usize>().unwrap_or(0)];
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/hotcomments/{}{}",
        _type, id
    );

    let limit = query.value("limit").unwrap_or("20");
    let offset = query.value("offset").unwrap_or("0");
    let beforeTime = query.value("beforeTime").unwrap_or("0");
    let t = &format!("rid={}&limit={}&offset={}&beforeTime={}",
           id, limit, offset, beforeTime
       );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/comment/hotwall/list")]
fn index_comment_hotwall_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/comment/hotwall/list/get";

    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/comment/like")]
fn index_comment_like(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = query.value("t").unwrap();
    let url = format!("https://music.163.com/weapi/v1/comment/{}", t);

    let _type = resource_type[ query.value("type").unwrap().parse::<usize>().unwrap_or(0) ];
    let threadId: String = if _type == "A_EV_2" {
        String::from(query.value("threadId").unwrap())
    } else {
        _type.to_owned() + query.value("id").unwrap()
    };
    let commentId = query.value("cid").unwrap();

    let t = &format!("threadId={}&commentId={}", threadId, commentId);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/comment/music")]
fn index_comment_music(req: HttpRequest) ->impl Responder {
    let query = QueryParams::from(req.query_string());
    let rid = query.value("id").unwrap();
    let url = format!("https://music.163.com/api/v1/resource/comments/R_SO_4_{}", rid);

    let info = QueryParams::from(query);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/comment/mv")]
fn index_comment_mv(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let rid = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_MV_5_{}", rid);

    let info = QueryParams::from(query);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/comment/playlist")]
fn index_comment_playlist(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let rid = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/A_PL_0_{}", rid);

    let info = QueryParams::from( query );

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/comment/video")]
fn index_comment_video(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let rid = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_VI_62_{}", rid);

    let info = QueryParams::from( query );
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/daily_signin")]
fn index_daily_signin(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/point/dailyTask";
    let info = QueryParams::from(req.query_string());
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/digitalAlbum/purchased")]
fn index_digitalAlbum_purchased(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/digitalAlbum/purchased";
    let query = QueryParams::from(req.query_string());
    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;

    let t = &format!("limit={}&offset={}&total={}",limit,offset,total);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/banner")]
fn index_dj_banner(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/banner/get";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/category/excludehot")]
fn index_dj_category_exclude_hot(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/category/excludehot";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/category/recommend")]
fn index_dj_category_recommend(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/home/category/recommend";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/catelist")]
fn index_dj_category_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/category/get";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/detail")]
fn index_dj_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/get";
    let info = QueryParams::from(req.query_string());

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/hot")]
fn index_dj_hot(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/hot/v1";
    let info = QueryParams::from(req.query_string());

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/toplist/pay")]
fn index_dj_pay_gift(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/home/paygift/list?_nmclfl=1";
    let info = QueryParams::from(req.query_string());

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/program/detail")]
fn index_dj_program_details(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/dj/program/detail";
    let info = QueryParams::from(req.query_string());

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/program/toplist")]
fn index_dj_program_toplist(req: HttpRequest) -> impl Responder {
   let url = "https://music.163.com/api/program/toplist/v1";
   let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/program/toplist/hours")]
fn index_dj_program_toplist_hours(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djprogram/toplist/hours";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/program")]
fn index_dj_program(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/dj/program/byradio";
    let query = QueryParams::from(req.query_string());
    let radioId = query.value("rid").unwrap();
    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");
    let asc = query.value("asc").unwrap_or("false").parse::<bool>().unwrap_or(false);

    let t = &format!("radioId={}&limit={}&offset={}&asc={}",
        radioId, limit, offset, asc
    );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/radio/hot")]
fn index_dj_radio_hot(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djradio/hot";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/recommend")]
fn index_dj_recommend(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/recommend/v1";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/recommend/type")]
fn index_dj_recommend_type(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/recommend";
    let query = QueryParams::from(req.query_string());
    let cateId = query.value("type").unwrap_or("10001");
    let t = &format!("cateId={}", cateId);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/sub")]
fn index_dj_sub(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = if query.value("t").unwrap_or("0").parse::<usize>().unwrap() == 1 {
        "sub"
    } else {
        "unsub"
    };
    let url = format!("https://music.163.com/weapi/djradio/{}", t);

    let t = &format!("id={}", query.value("rid").unwrap());
    let info = QueryParams::from( t );
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/sublist")]
fn index_dj_sub_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/get/subed";
    let info = QueryParams::from( req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/today/perfered")]
fn index_dj_today_perfered(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/home/today/perfered";
    let info = QueryParams::from( req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/toplist")]
fn index_dj_toplist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djradio/toplist";

    let query = QueryParams::from(req.query_string());
    let limit = query.value("limit").unwrap_or("100");
    let offset = query.value("offset").unwrap_or("0");
    let rtype = if query.value("type").unwrap_or("new") == "new" {
        0
    } else {
        1
    };

    let t = &format!("limit={}&offset={}&type={}",
        limit, offset, rtype
    );
    let info = QueryParams::from( t );
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/toplist/hours")]
fn index_dj_toplist_hours(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/dj/toplist/hours";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/toplist/newcomer")]
fn index_dj_toplist_newcomer(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/dj/toplist/newcomer";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/toplist/pay")]
fn index_dj_toplist_pay(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djradio/toplist/pay";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None,
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params,
            )
        )
}

#[get("/dj/toplist/popular")]
fn index_dj_toplist_popular(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/dj/toplist/popular";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/event")]
fn index_event(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/event/get";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/event/del")]
fn index_event_del(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/eapi/event/delete";
    let query = QueryParams::from(req.query_string());
    let t = &format!("id={}", query.value("evId").unwrap());
    let info = QueryParams::from( t );
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/event/forward")]
fn index_event_forward(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/event/forward";
    let query = QueryParams::from(req.query_string());
    let t = &format!("forwards={}&id={}&eventUserId={}",
        query.value("forwards").unwrap(),
        query.value("evId").unwrap(),
        query.value("uid").unwrap()
    );
    let info = QueryParams::from( t );
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/fm_trash")]
fn index_fm_trash(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let time = query.value("time").unwrap_or("25");
    let url = format!("https://music.163.com/weapi/radio/trash/add?alg=RT&songId={}&time={}", id, time);

    let t = &format!("songId={}", id);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/follow")]
fn index_follow(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = if query.value("t").unwrap_or("0") == "1" {
        "follow"
    } else {
        "delfollow"
    };
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/user/{}/{}",t,id);

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    let info = QueryParams::from("");

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/hot/topic")]
fn index_hot_topic(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/act/hot";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/like")]
fn index_like(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let alg = query.value("alg").unwrap_or("itembased");
    let id = query.value("id").unwrap();
    let time = query.value("time").unwrap_or("25");
    let url = format!(
        "https://music.163.com/weapi/radio/like?alg={}&trackId={}&time={}",
        alg, id, time
    );
    let t = &format!("trackId={}&like={}",
        id, query.value("like").unwrap_or("true")
    );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/likelist")]
fn index_likelist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/like/get";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/login")]
fn index_login(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/login";
    let query = QueryParams::from(req.query_string());
    let username = query.value("email").unwrap();
    let password = Crypto::hash_encrypt(
        query.value("password").unwrap(),
        HashType::md5,
        hex::encode
    );
    let rememberLogin = query.value("rememberLogin").unwrap_or("true");
    let t = &format!("username={}&password={}&rememberLogin={}",
        username, password, rememberLogin
    );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "pc",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params,
            )
        )
}

#[get("/login/cellphone")]
fn index_login_cellphone(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/login/cellphone";
    let query = QueryParams::from(req.query_string());
    let phone = query.value("phone").unwrap();
    let countrycode = query.value("countrycode").unwrap_or("86");
    let password = Crypto::hash_encrypt(
        query.value("password").unwrap(),
        HashType::md5,
        hex::encode
    );
    let rememberLogin = query.value("rememberLogin").unwrap_or("true");

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();

    let p =format!("phone={}&countrycode={}&password={}&rememberLogin={}",
                   phone, countrycode, password, rememberLogin
    );
    let info = QueryParams::from(&p);

    let params = RequestParams {
        ua: "pc",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/login/refresh")]
fn index_login_refresh(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/login/token/refresh";
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "pc",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &QueryParams::from("{}"),
                &params
            )
        )
}

#[get("/login/status")]
fn index_login_status(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com";
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &QueryParams::from(("")),
                &params
            )
        )
}

#[get("/logout")]
fn index_logout(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/logout";
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &QueryParams::from(""),
                &params
            )
        )
}

#[get("/lyric")]
fn index_lyric(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/lyric?lv=-1&kv=-1&tv=-1";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "linuxapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/msg/comments")]
fn index_msg_comments(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let uid = query.value("uid").unwrap();
    let url = format!("https://music.163.com/api/v1/user/comments/{})",uid);
    let beforeTime = query.value("before").unwrap_or("-1");
    let limit = query.value("limit").unwrap_or("30");
    let total = true;
    let t = &format!("beforeTime={}&limit={}&total={}&uid={}",
        beforeTime, limit, total, uid
    );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/msg/forwards")]
fn index_msg_forwards(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/forwards/get";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/msg/notices")]
fn index_msg_notices(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/msg/notices";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/msg/private")]
fn index_msg_private(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/msg/private/users";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/msg/private/history")]
fn index_msg_private_history(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/msg/private/history";
    let info = QueryParams::from(req.query_string())
            .replace_key("uid", "userId")
            .replace_key("before", "time");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/mv/all")]
fn index_mv_all(req: HttpRequest) -> impl Responder {
    let url = "https://interface.music.163.com/api/mv/all";

    let query = QueryParams::from(req.query_string());
    let area = query.value("area").unwrap_or("全部");
    let rtype = query.value("type").unwrap_or("全部");
    let order = query.value("order").unwrap_or("上升最快");
    let tags_value = format!(r#"{{"地区":"{}","类型":"{}","排序":"{}"}}"#,
        area, rtype, order
    );
    let tags = format!(r#"{{"tags":{}}}"#, tags_value);
    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;
    let t = &format!("tags={}&offset={}&total={}&limit={}",
        tags, offset, total, limit
    );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/mv/detail")]
fn index_mv_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/mv/detail";
    let info = QueryParams::from(req.query_string())
        .replace_key("mvid", "id");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/mv/exclusive/rcmd")]
fn index_mv_exclusive_rcmd(req: HttpRequest) -> impl Responder {
    let url = "https://interface.music.163.com/api/mv/exclusive/rcmd";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/mv/first")]
fn index_mv_first(req: HttpRequest) -> impl Responder {
    let url = "https://interface.music.163.com/weapi/mv/first";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/mv/sub")]
fn index_mv_sub(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = if query.value("t").unwrap_or("0") == "1" {
        "sub"
    } else {
        "unsub"
    };
    let url = format!("https://music.163.com/weapi/mv/{}", t);
    let id = query.value("mvid").unwrap();
    let t = &format!("[{}]", id);
    let info = query.replace_key("mvid", "mvIds")
        .replace_value(id, t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/mv/sublist")]
fn index_mv_sublist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/cloudvideo/allvideo/sublist";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/mv/url")]
fn index_mv_url(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/enhance/play/mv/url";

    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let r = query.value("res").unwrap_or("1080");

    let t = &format!("id={}&r={}", id, r);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/personal_fm")]
fn index_personal_fm(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/radio/get";
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &QueryParams::from(""),
                &params
            )
        )
}

#[get("/personalized")]
fn index_personalized(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/playlist";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/personalized/djprogram")]
fn index_personalized_djprogram(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/djprogram";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/personalized/mv")]
fn index_personalized_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/mv";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/personalized/newsong")]
fn index_personalized_newsong(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/newsong";
    let t = &format!("type=recommend");
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/personalized/privatecontent")]
fn index_personalized_privatecontent(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/privatecontent";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/playlist/catlist")]
fn index_playlist_catlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/catalogue";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/playlist/create")]
fn index_playlist_create(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/create";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/playlist/delete")]
fn index_playlist_delete(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/delete";
    let info = QueryParams::from(req.query_string())
        .replace_key("id","pid");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/playlist/desc/update")]
fn index_playlist_desc_update(req: HttpRequest) -> impl Responder {
    let url = "http://interface3.music.163.com/eapi/playlist/desc/update";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "eapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/playlist/detail")]
fn index_playlist_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v3/playlist/detail";

    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let s = query.value("s").unwrap_or("8");
    let t = &format!("id={}&n=100000&s={}", id, s);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "linuxapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/playlist/hot")]
fn index_playlist_hot(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/hottags";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/playlist/name/update")]
fn index_playlist_name_update(req: HttpRequest) -> impl Responder {
    let url = "http://interface3.music.163.com/eapi/playlist/update/name";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "eapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/playlist/subscribe")]
fn index_playlist_subscribe(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = if query.value("t").unwrap_or("0") == "1" {
        "subscribe"
    } else {
        "unsubscribe"
    };
    let url = format!("https://music.163.com/weapi/playlist/{}", t);
    let t = &format!("id={}", query.value("id").unwrap());
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/playlist/subscribers")]
fn index_playlist_subscribers(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/subscribers";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "eapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )

}

#[get("/playlist/tags/update")]
fn index_playlist_tags_update(req: HttpRequest) -> impl Responder {
    let url = "http://interface3.music.163.com/eapi/playlist/tags/update";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "eapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/playlist/tracks")]
fn index_playlist_tracks(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/manipulate/tracks";
    let query = QueryParams::from(req.query_string());
    let t = &format!("op={}&pid={}&trackIds=[{}]",
            query.value("op").unwrap(),
            query.value("pid").unwrap(),
            query.value("tracks").unwrap(),
        );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "eapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/playlist/update")]
fn index_playlist_update(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/batch";

    let query = QueryParams::from(req.query_string());
    let desc = query.value("desc").unwrap_or("");
    let tags = query.value("tags").unwrap_or("");
    let id = query.value("id").unwrap();
    let name = query.value("name").unwrap();

    let desc_update = format!(r#"{{"id":{},"desc":"{}"}}"#, id, desc);
    let tags_update = format!(r#"{{"id":{},"tags":"{}"}}"#, id, tags);
    let name_update = format!(r#"{{"id":{},"name":"{}"}}"#, id, name);
    let t = &format!(r#"/api/playlist/desc/update={}&/api/playlist/tags/update={}&/api/playlist/update/name={}"#,
            desc_update, tags_update, name_update
        );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/playmode/intelligence/list")]
fn index_playmode_interlligence_list(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/playmode/intelligence/list";

    let query = QueryParams::from(req.query_string());
    let songId = query.value("id").unwrap();
    let _type = "fromPlayOne";
    let playlistId = query.value("pid").unwrap();
    let startMusicId = if let Some(sId) = query.value("sid") {
        sId
    } else {
        songId
    };
    let count = query.value("count").unwrap_or("1");
    let t = &format!("songId={}&type={}&playlistId={}&startMusicId={}&count={}",
            songId, _type, playlistId, startMusicId, count
        );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/program/recommend")]
fn index_program_recommend(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/program/recommend/v1";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/rebind")]
fn index_rebind(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/user/replaceCellphone";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/recommend/resource")]
fn index_recommend_resource(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/recommend/resource";
    let info = QueryParams::from("");
    println!("info={:?}", info);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/recommend/songs")]
fn index_recommend_songs(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/recommend/songs";
    let t = &format!("total={}", true );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    println!("cookie={:?}", cookies);
    println!("headers={:?}", req.headers());
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/register/cellphone")]
fn index_register_cellphone(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/register/cellphone";
    let query = QueryParams::from(req.query_string());
    let pw = query.value("password").unwrap();
    let t = &Crypto::hash_encrypt(pw, HashType::md5, hex::encode);
    let info = query.replace_value(
        pw,
        t
    );
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/related/allvideo")]
fn index_related_allvideo(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/cloudvideo/v1/allvideo/rcmd";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/related/playlist")]
fn index_related_playlist(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/playlist?id={}", id);
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/resource/like")]
fn index_resource_like(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = if query.value("t").unwrap_or("0") == "1" {
        "like"
    } else {
        "unlike"
    };
    let url = format!("https://music.163.com/weapi/resource/{}", t);

    let rtype = resource_type[
        query.value("t").unwrap().parse::<usize>().unwrap_or(0)
        ];

    let threadId = if rtype == "A_EV_2_" {
        String::from(query.value("threadId").unwrap())
    } else {
        rtype.to_owned() + query.value("id").unwrap()
    };

    let t = &format!("threadId={}", threadId);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/scrobble")]
fn index_scrobble(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/feedback/weblog";

    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let sourceId = query.value("sourceid").unwrap();
    let time = query.value("time").unwrap();

    let json_value = format!(
        r##"{{"download":"0","end":"playend","id":"{}","sourceId":"{}","time":"{}","type":"song","wifi":"0"}}"##,
        id,sourceId,time
    );
    let log_value = format!(r#"[{{"action":"play","json":"{}"}}]"#,
        json_value
    );
    let t = &format!("log={}", log_value);
    let info = QueryParams::from(t);

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/search")]
fn index_search(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/search/get";

    let query = QueryParams::from(req.query_string());
    let s = query.value("keywords").unwrap();
    let rtype = query.value("type").unwrap_or("1");
    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");
    let t = &format!("s={}&type={}&limit={}&offset={}",
             s, rtype, limit, offset
        );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/search/default")]
fn index_search_default(req: HttpRequest) -> impl Responder{
    let url = "http://interface3.music.163.com/eapi/search/defaultkeyword/get";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "eapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/search/hot")]
fn index_search_hot(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/search/hot";
    let value = QueryParams::from("type=1111");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    let info = QueryParams::from("");
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/search/hot/detail")]
fn index_search_hot_detail(req: HttpRequest) -> impl Responder{
    let url = "https://music.163.com/weapi/hotsearchlist/get";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/search/multimatch")]
fn index_search_multimatch(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/search/suggest/multimatch";
    let info = QueryParams::from(req.query_string())
        .replace_key("keywords","s");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/search/suggest")]
fn index_search_suggest(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let rtype = if query.value("type").unwrap_or("web") == "mobile" {
        "keyword"
    } else {
        "web"
    };

    let url = format!("https://music.163.com/weapi/search/suggest/{}", rtype);
    let s = query.value("keywords").unwrap();
    let t = &format!("s={}", s);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/send/playlist")]
fn index_send_playlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/msg/private/send";

    let query = QueryParams::from(req.query_string());
    let id = query.value("playlist").unwrap();
    let msg = query.value("msg").unwrap();
    let userIds = query.value("user_ids").unwrap();

    let t= &format!("id={}&type=playlist&msg={}&userIds=[{}]",id,msg,userIds);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/send/text")]
fn index_send_text(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/msg/private/send";

    let query = QueryParams::from(req.query_string());
    let id = query.value("playlist").unwrap();
    let msg = query.value("msg").unwrap();
    let userIds = query.value("user_ids").unwrap();

    let t = &format!("id={}&type=text&msg={}&userIds=[{}]",id,msg,userIds);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/setting")]
fn index_setting(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/user/setting";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/share/resource")]
fn index_share_resource(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/share/friends/resource";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/simi/artist")]
fn index_simi_artist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiArtist";
    let info = QueryParams::from(req.query_string())
        .replace_key("id", "artistid");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/simi/mv")]
fn index_simi_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiMV";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/simi/playlist")]
fn index_simi_playlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiPlaylist";
    let info = QueryParams::from(req.query_string())
        .replace_key("id", "songid");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/simi/song")]
fn index_simi_song(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/simiSong";
    let info = QueryParams::from(req.query_string())
        .replace_key("id", "songid");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/simi/user")]
fn index_simi_user(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiUser";
    let info = QueryParams::from(req.query_string())
        .replace_key("id", "songid");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/song/detail")]
fn index_song_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v3/song/detail";
    let query = QueryParams::from(req.query_string());
//    let ids: Vec<usize> = query.value("ids")
//        .unwrap().split(",").collect::<Vec<usize>>();
//
//    let info = QueryParams::from(
//        &format!("c=[{}]&ids=[]",
//            ids.iter().map(|id| {
//                format!(r#"{{"id":"{}"}}"#, id)
//            }
//            ).collect(),
//            //ids
//        )
//    );
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/song/url")]
fn index_song_url(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/song/enhance/player/url";
    let mut query = QueryParams::from(req.query_string())
        .replace_key("id", "ids");
    println!("query={}", req.query_string());
    if query.value("br") == None {
        println!("add query string: br:999000");
        query = query.add_query_string("br", "999000");
    } else {
        println!("query string br is already added");
    }
    let ids = query.value("ids").unwrap();
    let t = &format!("[{}]", ids);
    let info = query.replace_value(
        ids,
        t
    );
    println!("info={:?}; query={:?}", info, query);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "linuxapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/top/album")]
fn index_top_album(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/album/new";

    let query = QueryParams::from(req.query_string());
    let area = query.value("type").unwrap_or("ALL");
    let limit = query.value("limit").unwrap_or("50");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;
    let t= &format!("area={}&limit={}&offset={}&total={}",
            area, limit, offset, total
        );
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/top/artists")]
fn index_top_artists(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/top";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/top/list")]
fn index_top_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v3/playlist/detail";
    let query = QueryParams::from(req.query_string());
    let id = topList[query.value("idx").unwrap_or("0").parse::<usize>().unwrap()];
    let t= &format!("id={}&n=10000", id);
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "linuxapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/top/mv")]
fn index_top_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/mv/toplist";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/top/playlist")]
fn index_top_playlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/list";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/top/playlist/highquality")]
fn index_top_playlist_highquality(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/highquality/list";

    let query = QueryParams::from(req.query_string());
    let cat = query.value("cat").unwrap_or("全部");
    let limit = query.value("limit").unwrap_or("50");
    let lasttime = query.value("before").unwrap_or("0");
    let total = true;

    let t = &format!("cat={}&limit={}&lasttime={}&total={}", cat, limit, lasttime, total);
    let info = QueryParams::from(t);

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/top/song")]
fn index_top_song(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/new/songs";
    let t = &format!("areaId={}&total=true", QueryParams::from(req.query_string()).value("type").unwrap_or("0"));
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/toplist")]
fn index_toplist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/toplist";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/toplist/artist")]
fn index_toplist_artist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/toplist/artist";
    let t = &format!("type=1&limit=100&offset=0&total=true");
    let info = QueryParams::from(t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/toplist/detail")]
fn index_toplist_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/toplist/detail";
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "linuxapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/user/audio")]
fn index_user_audio(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/get/byuser";
    let info = QueryParams::from(req.query_string())
        .replace_key("uid", "userId");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}


#[get("/user/cloud")]
fn index_user_cloud(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/cloud/get";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("user/cloud/del")]
fn index_user_cloud_del(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/cloud/del";
    let query = QueryParams::from(req.query_string())
        .replace_key("id", "songIds");
    let songIds = query.value("songIds").unwrap();
    let t = &format!("[{}]", songIds);
    let info = query.replace_value(songIds, t);
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/user/cloud/detail")]
fn index_user_cloud_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/cloud/get/byids";
    let query = QueryParams::from(req.query_string());
    let queryValue = query.value("id").unwrap();

    let info = urlqstring::proto_object!({
        "songIds": queryValue.split(',').collect::<Vec<&str>>()
    });
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/user/detail")]
fn index_user_detail(req: HttpRequest) -> impl Responder {
    let url = format!("https://music.163.com/weapi/v1/user/detail/{}",
        QueryParams::from(req.query_string()).value("uid").unwrap()
    );
    let info = QueryParams::from("");
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}


#[get("/user/dj")]
fn index_user_dj(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = format!("https://music.163.com/weapi/dj/program/{}",
        query.value("uid").unwrap()
    );

    let info = urlqstring::proto_object!({
        "limit": query.value("limit").unwrap_or("30"),
        "offset": query.value("offset").unwrap_or("0")
    });

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/user/event")]
fn index_user_event(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = format!("https://music.163.com/weapi/event/get/{}",
        query.value("uid").unwrap()
    );

    let info = urlqstring::proto_object!({
        "getcounts" : "true",
        "time" : query.value("lasttime").unwrap_or("-1"),
        "limit" : query.value("limit").unwrap_or("30"),
        "total" : "true"
    });

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/user/followeds")]
fn index_user_followeds(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let uid = query.value("uid").unwrap();
    let url = format!("https://music.163.com/eapi/user/getfolloweds/{}",
        uid
    );

    let info = urlqstring::proto_object!({
        "userId": uid,
        "time": query.value("lasttime").unwrap_or("-1"),
        "limit": query.value("limit").unwrap_or("30")
    });

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/user/follows")]
fn index_user_follows(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = format!("https://music.163.com/weapi/user/getfollows/{}",
        query.value("uid").unwrap()
    );

    let info = urlqstring::proto_object!({
        "offset": query.value("offset").unwrap_or("0"),
        "limit": query.value("limit").unwrap_or("30"),
        "order": "true"
    });
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/user/playlist")]
fn index_user_playlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/user/playlist";
    let info = QueryParams::from(
        QueryParams::from(req.query_string())
    );
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/user/record")]
fn index_user_record(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/play/record";
    let info = QueryParams::from(
        QueryParams::from(req.query_string())
    );
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/user/subcount")]
fn index_user_subcount(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/subcount";
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &QueryParams::from(""),
                &params
            )
        )
}

#[get("/user/update")]
fn index_user_update(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/user/profile/update";
    let query = QueryParams::from(req.query_string());
    let info = urlqstring::proto_object!({
        "avatarImgId": "0",
        "birthday": query.value("birthday").unwrap(),
        "city": query.value("city").unwrap(),
        "gender": query.value("gender").unwrap(),
        "nickname": query.value("nickname").unwrap(),
        "province": query.value("province").unwrap(),
        "signature": query.value("signature").unwrap()
    });
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/video/detail")]
fn index_video_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/cloudvideo/v1/video/detail";
    let info = QueryParams::from(req.query_string());
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/video/group")]
fn index_video_group(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/videotimeline/videogroup/get";
    let query = QueryParams::from(req.query_string());
    let info = urlqstring::proto_object!({
        "groupId": query.value("id").unwrap(),
        "offset": query.value("offset").unwrap_or("0"),
        "needUrl": true,
        "resolution": query.value("res").unwrap_or("1080")
    });
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

#[get("/video/group/list")]
fn index_video_group_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/cloudvideo/group/list";
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &QueryParams::from(""),
                &params
            )
        )
}

#[get("/video/sub")]
fn index_video_sub(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = format!("https://music.163.com/weapi/cloudvideo/video/{}",
        query.value("t").unwrap()
    );
    let info = urlqstring::proto_object!({
        "id" : query.value("id").unwrap()
    });
    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                &url,
                &info,
                &params
            )
        )
}

#[get("/video/url")]
fn index_video_url(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/cloudvideo/playurl";
    let query = QueryParams::from(req.query_string());
    let info = urlqstring::proto_object!({
        "ids": format!("{:?}", query.value("id").unwrap().split(',').collect::<Vec<&str>>()),
        "resolution": query.value("res").unwrap_or("1080")
    });

    let cookies = req.cookies().unwrap();
    let cookies = cookies.to_hashmap();
    let params = RequestParams {
        ua: "",
        crypto: "weapi",
        cookies: cookies,
        url: None
    };
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                url,
                &info,
                &params
            )
        )
}

pub fn start_server() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
           .wrap(middleware::Logger::default())
           .service(index_activate_init_profile)
           .service(index_album)
           .service(index_album_detail_dynamic)
           .service(index_album_newest)
           .service(index_album_sub)
           .service(index_album_sublist)
           .service(index_artist_album)
           .service(index_artist_desc)
           .service(index_artist_list)
           .service(index_artist_mv)
           .service(index_artist_sub)
           .service(index_artist_sublist)
           .service(index_artist_top_song)
           .service(index_banner)
           .service(index_batch)
           .service(index_captcha_register)
           .service(index_captcha_sent)
           .service(index_captcha_verify)
           .service(index_cellphone_existence_check)
           .service(index_check_music)
           .service(index_comment)
           .service(index_comment_album)
           .service(index_comment_dj)
           .service(index_comment_event)
           .service(index_comment_hot)
           .service(index_comment_hotwall_list)
           .service(index_comment_like)
           .service(index_comment_music)
           .service(index_comment_mv)
           .service(index_comment_playlist)
           .service(index_comment_video)
           .service(index_daily_signin)
           .service(index_digitalAlbum_purchased)
           .service(index_dj_banner)
           .service(index_dj_category_exclude_hot)
           .service(index_dj_category_recommend)
           .service(index_dj_category_list)
           .service(index_dj_detail)
           .service(index_dj_hot)
           .service(index_dj_pay_gift)
           .service(index_dj_program)
           .service(index_dj_program_details)
           .service(index_dj_program_toplist)
           .service(index_dj_program_toplist_hours)
           .service(index_dj_radio_hot)
           .service(index_dj_recommend)
           .service(index_dj_recommend_type)
           .service(index_dj_sub)
           .service(index_dj_sub_list)
           .service(index_dj_today_perfered)
           .service(index_dj_toplist)
           .service(index_dj_toplist_hours)
           .service(index_dj_toplist_newcomer)
           .service(index_dj_toplist_pay)
           .service(index_dj_toplist_popular)
           .service(index_event)
           .service(index_event_del)
           .service(index_event_forward)
           .service(index_fm_trash)
           .service(index_follow)
           .service(index_hot_topic)
           .service(index_like)
           .service(index_likelist)
           .service(index_login)
           .service(index_login_cellphone)
           .service(index_login_refresh)
           .service(index_login_status)
           .service(index_logout)
           .service(index_lyric)
           .service(index_msg_comments)
           .service(index_msg_forwards)
           .service(index_msg_notices)
           .service(index_msg_private)
           .service(index_msg_private_history)
           .service(index_mv_all)
           .service(index_mv_detail)
           .service(index_mv_exclusive_rcmd)
           .service(index_mv_first)
           .service(index_mv_sub)
           .service(index_mv_sublist)
           .service(index_mv_url)
           .service(index_personal_fm)
           .service(index_personalized)
           .service(index_personalized_djprogram)
           .service(index_personalized_mv)
           .service(index_personalized_newsong)
           .service(index_personalized_privatecontent)
           .service(index_playlist_catlist)
           .service(index_playlist_create)
           .service(index_playlist_delete)
           .service(index_playlist_desc_update)
           .service(index_playlist_detail)
           .service(index_playlist_hot)
           .service(index_playlist_name_update)
           .service(index_playlist_subscribe)
           .service(index_playlist_subscribers)
           .service(index_playlist_tags_update)
           .service(index_playlist_tracks)
           .service(index_playlist_update)
           .service(index_playmode_interlligence_list)
           .service(index_program_recommend)
           .service(index_rebind)
           .service(index_recommend_resource)
           .service(index_recommend_songs)
           .service(index_register_cellphone)
           .service(index_related_allvideo)
           .service(index_related_playlist)
           .service(index_resource_like)
           .service(index_scrobble)
           .service(index_search)
           .service(index_search_default)
           .service(index_search_hot)
           .service(index_search_hot_detail)
           .service(index_search_multimatch)
           .service(index_search_suggest)
           .service(index_send_playlist)
           .service(index_send_text)
           .service(index_setting)
           .service(index_share_resource)
           .service(index_simi_artist)
           .service(index_simi_mv)
           .service(index_simi_playlist)
           .service(index_simi_song)
           .service(index_simi_user)
           .service(index_song_detail)
           .service(index_song_url)
           .service(index_top_album)
           .service(index_top_artists)
           .service(index_top_list)
           .service(index_top_mv)
           .service(index_top_playlist)
           .service(index_top_playlist_highquality)
           .service(index_top_song)
           .service(index_toplist)
           .service(index_toplist_artist)
           .service(index_toplist_detail)
           .service(index_user_audio)
           .service(index_user_cloud)
           .service(index_user_cloud_del)
           .service(index_user_cloud_detail)
           .service(index_user_detail)
           .service(index_user_dj)
           .service(index_user_event)
           .service(index_user_followeds)
           .service(index_user_follows)
           .service(index_user_playlist)
           .service(index_user_record)
           .service(index_user_subcount)
           .service(index_user_update)
           .service(index_video_detail)
           .service(index_video_group)
           .service(index_video_group_list)
           .service(index_video_sub)
           .service(index_video_url)
           .route("/", web::get().to(index_root))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        dbg!("server runing @ http://127.0.0.1:3000");
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run().unwrap();
}

#[cfg(test)]
mod tests {

   use serde_json::Serializer;
   use urlqstring::proto_object;
   use urlqstring::QueryParams;

   #[test]
   fn it_works() {
       let str = "id=5374627,5374628,5374629";
       let value = QueryParams::from(str).value("id").unwrap();
       let value: Vec<&str> = value.split(',').collect();

       let s1 = format!("{:?}", value);
       println!("s1={}", s1);

       let s = proto_object!({
           "songIds":s1
       });

       println!("s={}", s);
   }
}