
#[macro_use]
mod macros;

mod crypto;
mod music_api;
mod request;
mod server;

use structopt::StructOpt;
use crate::server::{
    Opt,
    start_server
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    start_server(&opt).await
}
