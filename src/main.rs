pub mod utils;

use utils::ipstuff::IpAndPort;
use actix_web::{get, web, App, HttpServer, Responder, post, HttpResponse};

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello World!".to_string()
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/img")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = IpAndPort::new();

    HttpServer::new(|| {
        App::new().service(greet).service(hello).service(echo)
    })
    .bind((config.ip.as_str(), config.port))?
    .run()
    .await
}
