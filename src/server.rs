
use actix_web::{HttpServer, App, Resource, web, HttpResponse, get};

use crate::music_api::*;

pub(crate) async fn start_server() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(index_album_detail_dynamic)
            .service(index_album_newest)
            .service(index_album_sub)
            .service(index_album_sublist)
            .service(index_album)
            .service(index_artist_album)
            .service(index_artist_desc)
            .service(index_artist_list)
            .service(index_artist_mv)
            .service(index_artist_sub)
            .service(index_artist_sublist)
            .service(index_artist_top_song)
            .service(index_artists)
            .service(index_banner)
            .service(index_check_music)
            .service(index_comment_album)
            .service(index_comment_dj)
            .service(index_comment_event)
            .service(index_comment_hot)
            .service(index_comment_hotwall_list)
            .service(index_comment_like)
            .service(index_comment_music)
            .service(index_comment_mv)
            .service(index_comment_playlist)
            .service(index_comment)
            .service(index_daily_sigin)
            .service(index_dj_banner)
            .service(index_dj_category_excludehot)
            .service(index_dj_category_recommend)
            .service(index_dj_catelist)
            .service(index_dj_detail)
            .service(index_dj_hot)
            .service(index_dj_paygift)
            .service(index_dj_program_detail)
            .service(index_dj_program_toplist_hours)
            .service(index_dj_program_toplist)
            .service(index_dj_program)
            .service(index_dj_radio_hot)
            .service(index_dj_recommend_type)
            .service(index_dj_recommend)
            .service(index_dj_sub)
            .service(index_dj_sublist)
            .service(index_dj_today_perfered)
            .service(index_dj_toplist_hours)
            .service(index_dj_toplist_newcomer)
            .service(index_dj_toplist_pay)
            .service(index_dj_toplist_popular)
            .service(index_dj_toplist)
            .service(index_event_del)
            .service(index_event_forward)
            .service(index_event)
            .service(index_fm_trash)
            .service(index_follow)
            .service(index_hot_topic)
            .service(index_like)
            .service(index_likelist)
            .service(index_login_cellphone)
            .service(index_login_refresh)
            .service(index_logout)
            .service(index_lyric)
            .service(index_msg_comments)
            .service(index_msg_forwards)
            .service(index_msg_notices)
            .service(index_msg_private_history)
            .service(index_msg_private)
            .service(index_mv_all)
            .service(index_mv_detail)
            .service(index_exclusive_rcmd)
            .service(index_mv_first)
            .service(index_mv_sub)
            .service(index_mv_sublist)
            .service(index_mv_url)
            .service(index_search)
            .service(index_song_url)
    });

    server.bind("localhost:8000")?.run().await
}