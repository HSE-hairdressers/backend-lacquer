use actix_web::{get, web, App, HttpServer, Responder};
use std::env;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

struct IpStuff {
    ip: String,
    port: u16,
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {



    let _ip = match env::var("IP_ADDRESS") {
        Ok(v) => v,
        Err(_) => panic!("$IP_ADDRESS is not set"),
    };
    let _port = match env::var("PORT") {
        Ok(v) => v,
        Err(_) => panic!("$PORT is not set"),
    };
    let config = IpStuff { ip: _ip, port: _port.parse().unwrap()};

    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind((config.ip.as_str(), config.port))?
    .run()
    .await
}
