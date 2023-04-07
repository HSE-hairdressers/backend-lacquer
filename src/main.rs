pub mod content;
pub mod repository;
pub mod server;
pub mod utils;

use actix_web::{App, HttpServer};
use content::{img, login, registration, sys_info};
// use log::LevelFilter;
use simple_logger::SimpleLogger;
use utils::ipstuff::IpAndPort;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .with_colors(true)
        .init()
        .unwrap();
    let config = IpAndPort::new();

    HttpServer::new(|| {
        App::new()
            .service(img)
            .service(sys_info)
            .service(login)
            .service(registration)
    })
    .bind((config.ip.as_str(), config.port))?
    .run()
    .await
}
