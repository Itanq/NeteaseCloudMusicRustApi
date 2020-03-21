
use actix_web::{HttpResponse, HttpServer, HttpRequest, Responder, HttpMessage, get, web, App};
use urlqstring::QueryParams;
use percent_encoding::percent_decode_str;
use std::borrow::Cow;

use crate::crypto::{Crypto, HashType};

use crate::request::generate_response;
use std::collections::HashMap;

fn get_cookie_string(req: &HttpRequest) -> String {
    req.cookies().unwrap().iter().fold(String::from(""),|acc,val| {
        val.to_string() + ";" + &acc
    })
}

async fn empty_query_params_handler(url: &str,  crypto: &str, req: HttpRequest) -> impl Responder {
    let query_params = json_object!({});

    let cookies = get_cookie_string(&req);
    let request_params = json_object!({
        "crypto": crypto,
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

async fn request_handler(
    url: &str,
    crypto: &str,
    query_params: HashMap<&str,&str>,
    cookies: &str,
    req: &HttpRequest) -> impl Responder
{

    let request_params = json_object!({
        "crypto": crypto,
        "cookie": cookies,
        "proxy": ""
    });

    generate_response(
        url,
        "POST",
        query_params,
        request_params
    ).await
}

#[get("/album/detail/dynamic")]
pub(crate) async fn index_album_detail_dynamic(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/album/detail/dynamic";
    let cookies = get_cookie_string(&req);
    let query_string = QueryParams::from(req.query_string());

    let query_params = json_object!({
        "id": query_string.value("id").unwrap()
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/album/newest")]
pub(crate) async fn index_album_newest(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/discovery/newAlbum";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/album/sub")]
pub(crate) async fn index_album_sub(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let sub = query_string.value("t").unwrap_or("0").parse::<i32>().unwrap();
    let url = &format!("https://music.163.com/api/album/{}", if sub == 1 { "sub" } else { "unsub" });

    let cookies = get_cookie_string(&req);
    let query_params = json_object!({
        "id": query_string.value("id").unwrap()
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/album/sublist")]
pub(crate) async fn index_album_sublist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/album/sublist";
    let cookies = get_cookie_string(&req);
    let query_string = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "limit": query_string.value("limit").unwrap_or("25"),
        "offset": query_string.value("offset").unwrap_or("0"),
        "total": "true"
    });
    let cookie = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookie, &req).await
}

#[get("/album")]
pub(crate) async fn index_album(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/weapi/v1/album/{}", query_string.value("id").unwrap_or("0"));
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/artist/album")]
pub(crate) async fn index_artist_album(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/weapi/artist/albums/{}", query_string.value("id").unwrap_or("0"));

    let cookies = get_cookie_string(&req);
    let query_params = json_object!({
        "limit": query_string.value("limit").unwrap_or("30"),
        "offset": query_string.value("offset").unwrap_or("0"),
        "total": "true"
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/artist/desc")]
pub(crate) async fn index_artist_desc(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/introduction";
    let query_string = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "id": query_string.value("id").unwrap()
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

// 歌手分类
/*
    categoryCode 取值
    入驻歌手 5001
    华语男歌手 1001
    华语女歌手 1002
    华语组合/乐队 1003
    欧美男歌手 2001
    欧美女歌手 2002
    欧美组合/乐队 2003
    日本男歌手 6001
    日本女歌手 6002
    日本组合/乐队 6003
    韩国男歌手 7001
    韩国女歌手 7002
    韩国组合/乐队 7003
    其他男歌手 4001
    其他女歌手 4002
    其他组合/乐队 4003
    initial 取值 a-z/A-Z
*/

#[get("/artist/list")]
pub(crate) async fn index_artist_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/list";
    let query_string = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "categoryCode": query_string.value("cat").unwrap_or("1001"),
        "initial": "undefined",
        "offset": query_string.value("offset").unwrap_or("0"),
        "total": "true"
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/artist/mv")]
pub(crate) async fn index_artist_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/mvs";
    let query_string = QueryParams::from(req.query_string());

    let query_params = json_object!({
        "artistId": query_string.value("id").unwrap(),
        "limit": query_string.value("limit").unwrap_or("25"),
        "offset": query_string.value("offset").unwrap_or("0"),
        "total": "true"
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/artist/sub")]
pub(crate) async fn index_artist_sub(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let sub = query_string.value("t").unwrap_or("0").parse::<i32>().unwrap();
    let url = &format!("https://music.163.com/weapi/artist/{}", if sub == 1 { "sub" } else { "unsub" });

    let ids = "[".to_owned() + query_string.value("id").unwrap() + "]";
    let query_params = json_object!({
        "artistId": query_string.value("id").unwrap(),
        "artistIds": &ids,
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/artist/sublist")]
pub(crate) async fn index_artist_sublist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/sublist";
    let query_string = QueryParams::from(req.query_string());

    let query_params = json_object!({
        "limit": query_string.value("cat").unwrap_or("25"),
        "offset": query_string.value("offset").unwrap_or("0"),
        "total": "true"
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/artist/top/song")]
pub(crate) async fn index_artist_top_song(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/artist/top/song";
    let query_string = QueryParams::from(req.query_string());

    let query_params = json_object!({
        "id": query_string.value("id").unwrap()
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/artists")]
pub(crate) async fn index_artists(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/weapi/v1/artist/{}", query_string.value("id").unwrap());
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/banner")]
pub(crate) async fn index_banner(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let url = "https://music.163.com/api/v2/banner/get";
    let type_arr = ["pc", "android", "iphone", "ipad"];
    let query_params = json_object!({
        "clientType": type_arr[query_string.value("type").unwrap_or("0").parse::<usize>().unwrap()]
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/check/music")]
pub(crate) async fn index_check_music(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/enhance/player/url";

    let query_string = QueryParams::from(req.query_string());
    let ids = "[".to_owned() + query_string.value("id").unwrap() + "]";
    let query_params = json_object!({
        "ids": query_string.value("id").unwrap(),
        "br": query_string.value("br").unwrap_or("999000"),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

async fn comment_common(url: &str, req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "rid": query_string.value("id").unwrap(),
        "limit": query_string.value("limit").unwrap_or("20"),
        "offset": query_string.value("offset").unwrap_or("0"),
        "beforeTime": query_string.value("before").unwrap_or("0"),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/comment/album")]
pub(crate) async fn index_comment_album(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/weapi/v1/resource/comments/R_AL_3_{}", query_string.value("id").unwrap());
    comment_common(url, req).await
}

#[get("/comment/dj")]
pub(crate) async fn index_comment_dj(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/weapi/v1/resource/comments/A_DJ_1_{}", query_string.value("id").unwrap());
    comment_common(url, req).await
}

#[get("/comment/event")]
pub(crate) async fn index_comment_event(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/weapi/v1/resource/comments/{}", query_string.value("threadId").unwrap());
    let query_params = json_object!({
        "limit": query_string.value("limit").unwrap_or("20"),
        "offset": query_string.value("offset").unwrap_or("0"),
        "beforeTime": query_string.value("before").unwrap_or("0"),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/comment/hot")]
pub(crate) async fn index_comment_hot(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let _type:&str = ["R_SO_4_", "R_MV_5_", "A_PL_0_", "R_AL_3_", "A_DJ_1_", "R_VI_62_"][query_string.value("type").unwrap_or("0").parse::<usize>().unwrap()];
    let url = &format!("https://music.163.com/weapi/v1/resource/hotcomments/{}{}", _type, query_string.value("id").unwrap());
    comment_common(url, req).await
}

#[get("/comment/hotwall/list")]
pub(crate) async fn index_comment_hotwall_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/comment/hotwall/list/get";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/comment/like")]
pub(crate) async fn index_comment_like(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let like = if query_string.value("t").unwrap_or("0") == "1" {
        "like"
    } else {
        "unlike"
    };
    let url = &format!("https://music.163.com/weapi/v1/comment/{}", like);
    let _type:&str = ["R_SO_4_", "R_MV_5_", "A_PL_0_", "R_AL_3_", "A_DJ_1_", "R_VI_62_", "A_EV_2_"][query_string.value("type").unwrap_or("0").parse::<usize>().unwrap()];
    let thread_id = _type.to_owned() + query_string.value("id").unwrap();
    let query_params = json_object!({
        "commentId": query_string.value("cid").unwrap(),
        "threadId": &thread_id,
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/comment/music")]
pub(crate) async fn index_comment_music(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/api/v1/resource/comments/R_SO_4_{}",query_string.value("id").unwrap());
    comment_common(url, req).await
}

#[get("/comment/mv")]
pub(crate) async fn index_comment_mv(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/weapi/v1/resource/comments/R_MV_5_{}",query_string.value("id").unwrap());
    comment_common(url, req).await
}


#[get("/comment/playlist")]
pub(crate) async fn index_comment_playlist(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/weapi/v1/resource/comments/A_PL_0_{}",query_string.value("id").unwrap());
    comment_common(url, req).await
}

#[get("/comment")]
pub(crate) async fn index_comment(req: HttpRequest) -> impl Responder {
    let query_string = QueryParams::from(req.query_string());
    let _t = ["add", "delete", "reply"][query_string.value("t").unwrap_or("0").parse::<usize>().unwrap()];

    let url = &format!("https://music.163.com/weapi/resource/comments/{}", _t);

    let _type:&str = ["R_SO_4_", "R_MV_5_", "A_PL_0_", "R_AL_3_", "A_DJ_1_", "R_VI_62_", "A_EV_2_"][query_string.value("type").unwrap_or("0").parse::<usize>().unwrap()];

    let mut query_params = json_object!({});
    let _td = _type.to_owned() + query_string.value("id").unwrap();
    if _type == "A_EV_2_" {
        query_params.insert("threadId", query_string.value("threadId").unwrap());
    } else {
        query_params.insert("threadId", &_td);
    };
    if _t == "add" {
        query_params.insert("content", query_string.value("content").unwrap());
    } else if _t == "delete" {
        query_params.insert("commentId", query_string.value("commentId").unwrap());
    } else if _t == "reply" {
        query_params.insert("commentId", query_string.value("commentId").unwrap());
        query_params.insert("content", query_string.value("content").unwrap());
    };

    let cookies = get_cookie_string(&req) + ";os=pc;";
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/daily_signin")]
pub(crate) async fn index_daily_sigin(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/point/dailyTask";

    let query_string = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "type": query_string.value("type").unwrap_or("0"),
    });
    let cookies = get_cookie_string(&req);
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

#[get("/digitalAlbum/purchased")]
pub(crate) async fn index_digitalAlbum_purchased(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/digitalAlbum/purchased";

    let query_string = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "limit": query_string.value("limit").unwrap_or("30"),
        "offset": query_string.value("offset").unwrap_or("0"),
        "total": "true"
    });
    let cookies = get_cookie_string(&req);
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

#[get("/dj/banner")]
pub(crate) async fn index_dj_banner(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/banner/get";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/dj/category/excludehot")]
pub(crate) async fn index_dj_category_excludehot(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/category/excludehot";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/dj/category/recommend")]
pub(crate) async fn index_dj_category_recommend(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/home/category/recommend";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/dj/catelist")]
pub(crate) async fn index_dj_catelist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/category/get";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/dj/detail")]
pub(crate) async fn index_dj_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/get";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "id": query.value("rid").unwrap(),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/hot")]
pub(crate) async fn index_dj_hot(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/hot/v1";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "limit": query.value("limit").unwrap_or("30"),
        "offset": query.value("offset").unwrap_or("0"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/paygift")]
pub(crate) async fn index_dj_paygift(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/home/paygift/list?_nmclfl=1";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "limit": query.value("limit").unwrap_or("30"),
        "offset": query.value("offset").unwrap_or("0"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/program/detail")]
pub(crate) async fn index_dj_program_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/dj/program/detail";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "id": query.value("id").unwrap(),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/program/toplist/hours")]
pub(crate) async fn index_dj_program_toplist_hours(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djprogram/toplist/hours";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "limit": query.value("limit").unwrap_or("30"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/program/toplist")]
pub(crate) async fn index_dj_program_toplist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/program/toplist/v1";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "limit": query.value("limit").unwrap_or("100"),
        "offset": query.value("offset").unwrap_or("0"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/program")]
pub(crate) async fn index_dj_program(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/dj/program/byradio";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "radioId": query.value("rid").unwrap(),
        "limit": query.value("limit").unwrap_or("30"),
        "offset": query.value("offset").unwrap_or("0"),
        "asc": query.value("asc").unwrap_or("false")
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/radio/hot")]
pub(crate) async fn index_dj_radio_hot(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djradio/hot";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "cateId": query.value("cateId").unwrap(),
        "limit": query.value("limit").unwrap_or("30"),
        "offset": query.value("offset").unwrap_or("0"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

// 精选电台分类
/*
    有声书 10001
    知识技能 453050
    商业财经 453051
    人文历史 11
    外语世界 13
    亲子宝贝 14
    创作|翻唱 2001
    音乐故事 2
    3D|电子 10002
    相声曲艺 8
    情感调频 3
    美文读物 6
    脱口秀 5
    广播剧 7
    二次元 3001
    明星做主播 1
    娱乐|影视 4
    科技科学 453052
    校园|教育 4001
    旅途|城市 12
*/

#[get("/dj/recommend/type")]
pub(crate) async fn index_dj_recommend_type(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/recommend";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "cateId": query.value("type").unwrap(),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/recommend")]
pub(crate) async fn index_dj_recommend(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/recommend/v1";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/dj/sub")]
pub(crate) async fn index_dj_sub(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let sub = if query.value("t").unwrap_or("0") == "1" { "sub" } else { "unsub" };
    let url = &format!("https://music.163.com/weapi/djradio/{}", sub);
    let _params = json_object!({
        "id": query.value("rid").unwrap(),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/sublist")]
pub(crate) async fn index_dj_sublist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/get/subed";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "limit": query.value("limit").unwrap_or("30"),
        "offset": query.value("offset").unwrap_or("0"),
        "total": "true"
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/today/perfered")]
pub(crate) async fn index_dj_today_perfered(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/home/today/perfered";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "page": query.value("page").unwrap_or("0"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/toplist/hours")]
pub(crate) async fn index_dj_toplist_hours(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/dj/toplist/hours";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "limit": query.value("limit").unwrap_or("100"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/toplist/newcomer")]
pub(crate) async fn index_dj_toplist_newcomer(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/dj/toplist/newcomer";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "limit": query.value("limit").unwrap_or("100"),
        "offset": query.value("offset").unwrap_or("0"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/toplist/pay")]
pub(crate) async fn index_dj_toplist_pay(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djradio/toplist/pay";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "limit": query.value("limit").unwrap_or("100"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/toplist/popular")]
pub(crate) async fn index_dj_toplist_popular(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/dj/toplist/popular";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "limit": query.value("limit").unwrap_or("100"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/dj/toplist")]
pub(crate) async fn index_dj_toplist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djradio/toplist";
    let query = QueryParams::from(req.query_string());
    let _type = if query.value("type").unwrap_or("new") == "new" {
        "0"
    } else {
        "1"
    };
    let _params = json_object!({
        "limit": query.value("limit").unwrap_or("0"),
        "offset": query.value("offset").unwrap_or("0"),
        "type": _type
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/login/cellphone")]
pub(crate) async fn index_login_cellphone(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/login/cellphone";
    let query_string = QueryParams::from(req.query_string());
    let pw = Crypto::hash_encrypt(
        query_string.value("password").unwrap(),
        HashType::md5,
        hex::encode
    );
    let query_params = json_object!({
        "phone": query_string.value("phone").unwrap(),
        "countrycode": query_string.value("countrycode").unwrap_or("86"),
        "password": &pw,
        "rememberLogin": "true",
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/login/refresh")]
pub(crate) async fn index_login_refresh(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/login/token/refresh";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/song/url")]
pub(crate) async fn index_song_url(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/song/enhance/player/url";
    let query_string = QueryParams::from(req.query_string());

    let ids = "[".to_owned() + query_string.value("id").unwrap() + "]";
    let query_params = json_object!({
        "ids": ids.as_str(),
        "br": query_string.value("br").unwrap_or("999000")
    });

    let cookies = get_cookie_string(&req) + ";os=pc;";
    request_handler(url, "linuxapi", query_params, &cookies, &req).await
}

#[get("/search")]
pub(crate) async fn index_search(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/search/get";

    let qs = percent_decode_str(req.query_string())
        .decode_utf8().unwrap_or(Cow::Borrowed(&""));
    let query_string = QueryParams::from(qs.as_ref());
    let query_params = json_object!({
        "s": query_string.value("keywords").unwrap_or(""),
        "type": query_string.value("type").unwrap_or("1"),
        "limit": query_string.value("limit").unwrap_or("30"),
        "offset": query_string.value("offset").unwrap_or("0")
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}
