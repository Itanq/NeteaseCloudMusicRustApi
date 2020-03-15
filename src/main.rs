
#[macro_use]
mod macros;

mod crypto;
mod music_api;
mod request;
mod server;

use crate::server::start_server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    println!("Hello World!\n Start...");

    start_server().await
}
