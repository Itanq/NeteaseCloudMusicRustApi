
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

#[get("/event/del")]
pub(crate) async fn index_event_del(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/eapi/event/delete";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "id": query.value("evId").unwrap(),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/event/forward")]
pub(crate) async fn index_event_forward(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/event/forward";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "id": query.value("evId").unwrap(),
        "forwards": query.value("forwards").unwrap(),
        "eventUserId": query.value("uid").unwrap()
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/event")]
pub(crate) async fn index_event(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/event/get";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "pagesize": query.value("pagesize").unwrap_or("20"),
        "lasttime": query.value("lasttime").unwrap_or("-1"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/fm/trash")]
pub(crate) async fn index_fm_trash(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/weapi/radio/trash/add?alg=RT&songId={}&time={}", query.value("id").unwrap(), query.value("time").unwrap_or("25"));
    let _params = json_object!({
        "songId": query.value("id").unwrap(),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/follow")]
pub(crate) async fn index_follow(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let _t = if query.value("t").unwrap_or("0") == "1" { "follow" } else { "delfollow" };
    let url = &format!("https://music.163.com/weapi/user/{}/{}", _t, query.value("id").unwrap());
    let _params = json_object!({});
    let cookies = get_cookie_string(&req) + ";os=pc;";
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/hot/topic")]
pub(crate) async fn index_hot_topic(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/act/hot";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "limit": query.value("limit").unwrap_or("20"),
        "offset": query.value("offset").unwrap_or("0"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/like")]
pub(crate) async fn index_like(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = &format!(
        "https://music.163.com/weapi/radio/like?alg={}&trackId={}&time={}",
        query.value("alg").unwrap_or("itembased"),
        query.value("id").unwrap(),
        query.value("time").unwrap_or("25")
    );
    let _params = json_object!({
        "trackId": query.value("id").unwrap(),
        "like": query.value("like").unwrap_or("false")
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", _params, &cookies, &req).await
}

#[get("/likelist")]
pub(crate) async fn index_likelist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/like/get";
    let query = QueryParams::from(req.query_string());
    let _params = json_object!({
        "uid": query.value("uid").unwrap(),
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

#[get("/logout")]
pub(crate) async fn index_logout(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/logout";
    let query_params = json_object!({});
    let cookies = get_cookie_string(&req);
    let request_params = json_object!({
        "crypto": "weapi",
        "ua": "pc",
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

#[get("/lyric")]
pub(crate) async fn index_lyric(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/lyric?lv=-1&kv=-1&tv=-1";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "id": query.value("id").unwrap()
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "linuxapi", query_params, &cookies, &req).await
}

#[get("/msg/comments")]
pub(crate) async fn index_msg_comments(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/api/v1/user/comments/{}", query.value("uid").unwrap());
    let query_params = json_object!({
        "beforeTime": query.value("before").unwrap_or("-1"),
        "limit": query.value("limit").unwrap_or("30"),
        "total": "true",
        "uid": query.value("uid").unwrap(),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/msg/forwards")]
pub(crate) async fn index_msg_forwards(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/forwards/get";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "offset": query.value("offset").unwrap_or("0"),
        "limit": query.value("limit").unwrap_or("30"),
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/msg/notices")]
pub(crate) async fn index_msg_notices(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/msg/notices";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "offset": query.value("offset").unwrap_or("0"),
        "limit": query.value("limit").unwrap_or("30"),
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/msg/private/history")]
pub(crate) async fn index_msg_private_history(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/msg/private/history";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "userId": query.value("uid").unwrap(),
        "limit": query.value("limit").unwrap_or("30"),
        "time": query.value("before").unwrap_or("0"),
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/msg/private")]
pub(crate) async fn index_msg_private(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/msg/private/users";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "offset": query.value("offset").unwrap_or("0"),
        "limit": query.value("limit").unwrap_or("30"),
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/mv/all")]
pub(crate) async fn index_mv_all(req: HttpRequest) -> impl Responder {
    let url = "https://interface.music.163.com/api/mv/all";
    let query = QueryParams::from(req.query_string());
    let tags = &format!("地区:{};类型:{};排序:{}",
        query.value("area").unwrap_or("全部"),
        query.value("type").unwrap_or("全部"),
        query.value("order").unwrap_or("上升最快"),
    );
    let query_params = json_object!({
        "offset": query.value("offset").unwrap_or("0"),
        "limit": query.value("limit").unwrap_or("30"),
        "tags": tags,
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/mv/detail")]
pub(crate) async fn index_mv_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/mv/detail";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "id": query.value("mvid").unwrap()
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/mv/exclusive/rcmd")]
pub(crate) async fn index_mv_exclusive_rcmd(req: HttpRequest) -> impl Responder {
    let url = "https://interface.music.163.com/api/mv/exclusive/rcmd";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "offset": query.value("offset").unwrap_or("0"),
        "limit": query.value("limit").unwrap_or("30")
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/mv/first")]
pub(crate) async fn index_mv_first(req: HttpRequest) -> impl Responder {
    let url = "https://interface.music.163.com/weapi/mv/first";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "area": query.value("area").unwrap_or(""),
        "limit": query.value("limit").unwrap_or("30"),
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/mv/sub")]
pub(crate) async fn index_mv_sub(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let _t = if query.value("t").unwrap_or("0") == "1" { "sub" } else { "unsub" };
    let url = &format!("https://music.163.com/weapi/mv/{}", _t);
    let mv_ids = r#"[""#.to_owned() + query.value("mvid").unwrap() + r#""]"#;
    let query_params = json_object!({
        "mvId": query.value("mvId").unwrap(),
        "mvIds": &mv_ids,
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/mv/sublist")]
pub(crate) async fn index_mv_sublist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/cloudvideo/allvideo/sublist";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "limit": query.value("limit").unwrap_or("25"),
        "offset": query.value("offset").unwrap_or("0"),
        "total": "true"
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/mv/url")]
pub(crate) async fn index_mv_url(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/enhance/play/mv/url";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "id": query.value("id").unwrap(),
        "r": query.value("res").unwrap_or("1080"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/personal/fm")]
pub(crate) async fn index_personal_fm(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/radio/get";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/personalized/djprogram")]
pub(crate) async fn index_personalized_djprogram(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/djprogram";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/personalized/mv")]
pub(crate) async fn index_personalized_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/mv";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/personalized/newsong")]
pub(crate) async fn index_personalized_newsong(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/newsong";
    let query_params = json_object!({
        "type": "recommend",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/personalized/privatecontent")]
pub(crate) async fn index_personalized_privatecontent(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/privatecontent";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/personalized")]
pub(crate) async fn index_personalized(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/playlist";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "limit": query.value("limit").unwrap_or("30"),
        "total": "true",
        "n": "1000",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/playlist/catlist")]
pub(crate) async fn index_playlist_catlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/catalogue";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/playlist/create")]
pub(crate) async fn index_playlist_create(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/create";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "name": query.value("name").unwrap(),
        "privacy": query.value("privacy").unwrap(),
    });
    let cookies = get_cookie_string(&req) + ";os=pc;";
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/playlist/delete")]
pub(crate) async fn index_playlist_delete(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/delete";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "pid": query.value("id").unwrap(),
    });
    let cookies = get_cookie_string(&req) + ";os=pc;";
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/playlist/desc_update")]
pub(crate) async fn index_playlist_desc_update(req: HttpRequest) -> impl Responder {
    let url = "http://interface3.music.163.com/eapi/playlist/desc/update";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "pid": query.value("id").unwrap(),
        "desc": query.value("desc").unwrap(),
    });
    let cookies = get_cookie_string(&req) + ";os=pc;";
    let request_params = json_object!({
        "crypto": "eapi",
        "cookie": &cookies,
        "proxy": "",
        "url": "/api/playlist/desc/update",
    });
    generate_response(
        url,
        "POST",
        query_params,
        request_params
    ).await
}

#[get("/playlist/detail")]
pub(crate) async fn index_playlist_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v3/playlist/detail";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "id": query.value("id").unwrap(),
        "n": "100000",
        "s": query.value("s").unwrap_or("8"),
    });
    let cookies = get_cookie_string(&req) + ";os=pc;";
    request_handler(url, "linuxapi", query_params, &cookies, &req).await
}

#[get("/playlist/hot")]
pub(crate) async fn index_playlist_hot(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/hottags";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/playlist/name/update")]
pub(crate) async fn index_playlist_name_update(req: HttpRequest) -> impl Responder {
    let url = "http://interface3.music.163.com/eapi/playlist/update/name";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "id": query.value("id").unwrap(),
        "name": query.value("name").unwrap(),
    });
    let cookies = get_cookie_string(&req) + ";os=pc;";
    let request_params = json_object!({
        "crypto": "eapi",
        "cookie": &cookies,
        "proxy": "",
        "url": "/api/playlist/update/name",
    });
    generate_response(
        url,
        "POST",
        query_params,
        request_params
    ).await
}

#[get("/playlist/subscribe")]
pub(crate) async fn index_playlist_subscribe(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let _t = if query.value("t").unwrap_or("0") == "1" { "subscribe" } else { "unsubscribe" };
    let url = &format!("https://music.163.com/weapi/playlist/{}", _t);
    let query_params = json_object!({
        "id": query.value("id").unwrap(),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/playlist/subscribers")]
pub(crate) async fn index_playlist_subscribers(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/subscribers";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "id": query.value("id").unwrap(),
        "limit": query.value("limit").unwrap_or("20"),
        "offset": query.value("offset").unwrap_or("0"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/playlist/tags/update")]
pub(crate) async fn index_playlist_tags_update(req: HttpRequest) -> impl Responder {
    let url = "http://interface3.music.163.com/eapi/playlist/tags/update";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "id": query.value("id").unwrap(),
        "tags": query.value("tags").unwrap(),
    });
    let cookies = get_cookie_string(&req);
    let request_params = json_object!({
        "crypto": "eapi",
        "cookie": &cookies,
        "proxy": "",
        "url": "/api/playlist/tags/update",
    });
    generate_response(
        url,
        "POST",
        query_params,
        request_params
    ).await
}

#[get("/playlist/tracks")]
pub(crate) async fn index_playlist_tracks(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/manipulate/tracks";
    let query = QueryParams::from(req.query_string());
    let ids = "[".to_owned() + query.value("trakcs").unwrap() + "]";
    let query_params = json_object!({
        "op": query.value("op").unwrap(),
        "pid": query.value("pid").unwrap_or("20"),
        "tackIds": &ids,
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/playlist/update")]
pub(crate) async fn index_playlist_update(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/batch";
    let query = QueryParams::from(req.query_string());
    let _id = query.value("id").unwrap();
    let _desc = format!(r#"{{"id":{},"desc":"{}"}}"#,
        _id,
        query.value("desc").unwrap_or(""),
    );
    let _tags = format!(r#"{{"id":{},"tags":"{}"}}"#,
        _id,
        query.value("tags").unwrap_or(""),
    );
    let _name = format!(r#"{{"id":{},"name":"{}"}}"#,
        _id,
        query.value("name").unwrap_or("")
    );
    let query_params = json_object!({
        "/api/playlist/desc/update": &_desc[..],
        "/api/playlist/tags/update": &_tags[..],
        "/api/playlist/update/name": &_name[..],
    });
    let cookies = get_cookie_string(&req) + "os=pc;";
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/playmode/intelligence/list")]
pub(crate) async fn index_playmode_intelligence_list(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/playmode/intelligence/list";
    let query = QueryParams::from(req.query_string());
    let ids = "[".to_owned() + query.value("trakcs").unwrap() + "]";
    let query_params = json_object!({
        "songId": query.value("id").unwrap(),
        "type": "fromPlayOne",
        "playlistId": query.value("pid").unwrap(),
        "startMusicId": query.value("sid").unwrap_or(query.value("id").unwrap()),
        "count": query.value("count").unwrap_or("1"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/program/recommend")]
pub(crate) async fn index_program_recommend(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/program/recommend/v1";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "cateId": query.value("type").unwrap(),
        "limit": query.value("limit").unwrap_or("10"),
        "offset": query.value("offset").unwrap_or("0"),
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/rebind")]
pub(crate) async fn index_rebind(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/user/replaceCellphone";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "captcha": query.value("captcha").unwrap(),
        "phone": query.value("phone").unwrap(),
        "oldcaptcha": query.value("oldcaptcha").unwrap(),
        "ctcode": query.value("ctcode").unwrap_or("86"),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/recommend/resource")]
pub(crate) async fn index_recommend_resource(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/recommend/resource";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/recommend/songs")]
pub(crate) async fn index_recommend_songs(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/recommend/songs";
    let query_params = json_object!({
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/register/cellphone")]
pub(crate) async fn index_register_cellphone(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/register/cellphone";
    let query = QueryParams::from(req.query_string());
    let pw = Crypto::hash_encrypt(
        query.value("password").unwrap(),
        HashType::md5,
        hex::encode
    );
    let query_params = json_object!({
        "captcha": query.value("captcha").unwrap(),
        "phone": query.value("phone").unwrap(),
        "password": &pw,
        "nickname": query.value("nickname").unwrap(),
    });

    let cookies = get_cookie_string(&req) + "os=pc;";
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/related/allvideo")]
pub(crate) async fn index_related_allvideo(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/cloudvideo/v1/allvideo/rcmd";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "id": query.value("id").unwrap(),
        "type": "1",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/related/playlist")]
pub(crate) async fn index_related_playlist(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = &format!("https://music.163.com/playlist?id={}", query.value("id").unwrap());
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/resource/like")]
pub(crate) async fn index_resource_like(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let _t = if query.value("t").unwrap_or("0") == "1" { "like" } else { "unlike" };
    let url = &format!("https://music.163.com/weapi/resource/{}", _t);

    let _type = ["", "R_MV_5_", "", "", "A_DJ_1_", "R_VI_62_", "A_EV_2_"][query.value("type").unwrap_or("0").parse::<usize>().unwrap()];
    let _id = _type.to_owned() + query.value("id").unwrap();
    let query_params = json_object!({
        "threadId": &_id[..]
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/search/default")]
pub(crate) async fn index_search_default(req: HttpRequest) -> impl Responder {
    let url = "http://interface3.music.163.com/eapi/search/defaultkeyword/get";
    let query_params = json_object!({});
    let cookies = get_cookie_string(&req);
    let request_params = json_object!({
        "crypto": "eapi",
        "cookie": &cookies,
        "proxy": "",
        "url": "/api/search/defaultkeyword/get",
    });

    generate_response(
        url,
        "POST",
        query_params,
        request_params
    ).await
}

#[get("/search/hot/detail")]
pub(crate) async fn index_search_hot_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/hotsearchlist/get";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/search/hot")]
pub(crate) async fn index_search_hot(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/search/hot";
    let query_params = json_object!({
        "type": "1111",
    });
    let cookies = get_cookie_string(&req);
    let request_params = json_object!({
        "crypto": "weapi",
        "ua": "mobile",
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

#[get("/search/multimatch")]
pub(crate) async fn index_search_multimatch(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/search/suggest/multimatch";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "type": query.value("type").unwrap_or("1"),
        "s": query.value("keywords").unwrap_or(""),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/search/suggest")]
pub(crate) async fn index_search_suggest(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let _type = if query.value("type").unwrap_or("mobile") == "mobile" {
        "keyword"
    } else {
        "web"
    };
    let url = &format!("https://music.163.com/weapi/search/suggest/{}", _type);
    let query_params = json_object!({
        "s": query.value("keywords").unwrap_or(""),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
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

#[get("/send/playlist")]
pub(crate) async fn index_send_playlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/msg/private/send";
    let query = QueryParams::from(req.query_string());
    let _ids = "[".to_owned() + query.value("user_ids").unwrap() + "]";
    let query_params = json_object!({
        "id": query.value("playlist").unwrap(),
        "type": "playlist",
        "msg": query.value("msg").unwrap_or(""),
        "userIds": &_ids,
    });

    let cookies = get_cookie_string(&req) + "os=pc;";
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/send/text")]
pub(crate) async fn index_send_text(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/msg/private/send";
    let query = QueryParams::from(req.query_string());
    let _ids = "[".to_owned() + query.value("user_ids").unwrap() + "]";
    let query_params = json_object!({
        "id": query.value("playlist").unwrap(),
        "type": "text",
        "msg": query.value("msg").unwrap_or(""),
        "userIds": &_ids,
    });

    let cookies = get_cookie_string(&req) + "os=pc;";
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/setting")]
pub(crate) async fn index_setting(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/user/setting";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/share/resource")]
pub(crate) async fn index_share_resource(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/share/friends/resource";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "type": query.value("type").unwrap_or("song"),
        "msg": query.value("msg").unwrap_or(""),
        "id": query.value("id").unwrap_or(""),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/simi/artist")]
pub(crate) async fn index_simi_artist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiArtist";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "artistid": query.value("id").unwrap(),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/simi/mv")]
pub(crate) async fn index_simi_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiMV";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "mvid": query.value("mvid").unwrap(),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/simi/playlist")]
pub(crate) async fn index_simi_playlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiPlaylist";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "songid": query.value("id").unwrap(),
        "limit": query.value("limit").unwrap_or("50"),
        "offset": query.value("offset").unwrap_or("0"),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/simi/song")]
pub(crate) async fn index_simi_song(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/simiSong";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "songid": query.value("id").unwrap(),
        "limit": query.value("limit").unwrap_or("51"),
        "offset": query.value("offset").unwrap_or("0"),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/simi/user")]
pub(crate) async fn index_simi_user(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiUser";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "songid": query.value("id").unwrap(),
        "limit": query.value("limit").unwrap_or("52"),
        "offset": query.value("offset").unwrap_or("0"),
    });

    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/song/detail")]
pub(crate) async fn index_song_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v3/song/detail";
    let query = QueryParams::from(req.query_string());
    let c = &format!(r#""[{{"id":{}}}]""#, query.value("ids").unwrap());
    let ids = &format!(r#""[{}]""#, query.value("ids").unwrap());
    let query_params = json_object!({
        "c": &c[..],
        "ids": &ids[..],
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/song/url")]
pub(crate) async fn index_song_url(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/song/enhance/player/url";
    let query = QueryParams::from(req.query_string());

    let ids = "[".to_owned() + query.value("id").unwrap() + "]";
    let query_params = json_object!({
        "ids": ids.as_str(),
        "br": query.value("br").unwrap_or("999000")
    });

    let cookies = get_cookie_string(&req) + ";os=pc;";
    request_handler(url, "linuxapi", query_params, &cookies, &req).await
}

#[get("/top/album")]
pub(crate) async fn index_top_album(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/album/new";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "area": query.value("type").unwrap_or("ALL"),
        "limit": query.value("limit").unwrap_or("50"),
        "offset": query.value("offset").unwrap_or("0"),
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/top/artist")]
pub(crate) async fn index_top_artist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/top";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "limit": query.value("limit").unwrap_or("50"),
        "offset": query.value("offset").unwrap_or("0"),
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/top/list")]
pub(crate) async fn index_top_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v3/playlist/detail";
    let query = QueryParams::from(req.query_string());
    static topList: [&str;37] = [
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
        "3001890046" //云音乐ACG VOCALOID榜
    ];
    let query_params = json_object!({
        "id": topList[query.value("idx").unwrap_or("0").parse::<usize>().unwrap()],
        "n": "10000",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "linuxapi", query_params, &cookies, &req).await
}

#[get("/top/mv")]
pub(crate) async fn index_top_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/mv/toplist";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "area": query.value("area").unwrap_or(""),
        "limit": query.value("limit").unwrap_or("30"),
        "offset": query.value("offset").unwrap_or("0"),
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/top/playlist/highquality")]
pub(crate) async fn index_top_playlist_highquality(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/highquality/list";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "cat": query.value("cat").unwrap_or("全部"),
        "limit": query.value("limit").unwrap_or("30"),
        "lasttime": query.value("before").unwrap_or("0"),
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/top/playlist")]
pub(crate) async fn index_top_playlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/list";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "cat": query.value("cat").unwrap_or("全部"),
        "order": query.value("order").unwrap_or("hot"),
        "limit": query.value("limit").unwrap_or("30"),
        "lasttime": query.value("before").unwrap_or("0"),
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/top/song")]
pub(crate) async fn index_top_song(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/new/songs";
    let query = QueryParams::from(req.query_string());
    let query_params = json_object!({
        "areaId": query.value("type").unwrap_or("0"),
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/toplist/artist")]
pub(crate) async fn index_toplist_artist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/toplist/artist";
    let query_params = json_object!({
        "type": "1",
        "limit": "100",
        "offset": "0",
        "total": "true",
    });
    let cookies = get_cookie_string(&req);
    request_handler(url, "weapi", query_params, &cookies, &req).await
}

#[get("/toplist/detail")]
pub(crate) async fn index_toplist_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/toplist/detail";
    empty_query_params_handler(url, "weapi", req).await
}

#[get("/toplist")]
pub(crate) async fn index_toplist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/toplist";
    empty_query_params_handler(url, "weapi", req).await
}
