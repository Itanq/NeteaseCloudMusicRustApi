mod app;
pub mod crypto;
pub mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    app::start_server().await
}
