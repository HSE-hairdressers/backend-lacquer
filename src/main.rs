pub mod content;
pub mod server;
pub mod utils;

use actix_web::{App, HttpServer};
use content::{greet, hello, img, sys_info};
use utils::ipstuff::IpAndPort;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = IpAndPort::new();

    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(hello)
            .service(img)
            .service(sys_info)
    })
    .bind((config.ip.as_str(), config.port))?
    .run()
    .await
}
