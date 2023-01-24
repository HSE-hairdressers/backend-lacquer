use actix_web::{get, web, App, HttpServer, Responder, post, HttpResponse};
use std::env;

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


struct IpStuff {
    ip: String,
    port: u16,
}

impl IpStuff {
    fn new() -> Self {
        let mut res = IpStuff::default();
        if let Ok(a) = env::var("IP_ADDRESS") {
            res.ip = a;
        }
        if let Ok(a) = env::var("PORT") {
            if let Ok(b) = a.parse() {
                res.port = b;
            }
        }
        res
    } 

    fn default() -> Self {
        IpStuff{ 
            ip: "localhost".to_string(), 
            port: 8011,
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = IpStuff::new();

    HttpServer::new(|| {
        App::new().service(greet).service(hello).service(echo)
    })
    .bind((config.ip.as_str(), config.port))?
    .run()
    .await
}
