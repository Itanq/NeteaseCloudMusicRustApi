
use actix_web::{HttpServer, App, Resource, web, HttpResponse, get};

use crate::music_api::*;

pub(crate) async fn start_server() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(index_search)
            .service(index_song_url)
    });

    server.bind("localhost:8000")?.run().await
}