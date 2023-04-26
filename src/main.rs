pub mod content;
pub mod repository;
pub mod server;
pub mod utils;

use actix_web::{App, HttpServer};
use content::{get_hairdresser_info, img, login, registration, sys_info, upload_image};
use utils::ipstuff::IpAndPort;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = IpAndPort::new();
    HttpServer::new(|| {
        App::new()
            .service(img)
            .service(sys_info)
            .service(login)
            .service(registration)
            .service(get_hairdresser_info)
            .service(upload_image)
    })
    .bind((config.ip.as_str(), config.port))?
    .run()
    .await
}
