pub mod content;
pub mod repository;
pub mod server;
pub mod utils;

use actix_web::{App, HttpServer};
use content::{img, sys_info, login};
use utils::ipstuff::IpAndPort;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = IpAndPort::new();

    HttpServer::new(|| App::new().service(img).service(sys_info).service(login))
        .bind((config.ip.as_str(), config.port))?
        .run()
        .await
}
