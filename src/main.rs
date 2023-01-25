pub mod utils;

use actix_multipart::Multipart;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder, Result};
use futures_util::StreamExt as _;
use std::io::Write;
use utils::ipstuff::IpAndPort;
use uuid::Uuid;

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello World!".to_string()
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    println!("{name}");
    format!("Hello {name}!")
}

#[post("/img")]
async fn img(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        let _ = std::fs::create_dir("./tmp");
        let filepath = format!("./tmp/{filename}");

        // File::create is blocking operation, use threadpool
        let mut file = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            // println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk?));
            file = web::block(move || {
                file.write_all(if let Some(f) = &chunk.ok() { f } else { &[0] })
                    .map(|_| file)
            })
            .await??
        }
    }

    Ok(HttpResponse::Ok().into())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = IpAndPort::new();

    HttpServer::new(|| App::new().service(greet).service(hello).service(img))
        .bind((config.ip.as_str(), config.port))?
        .run()
        .await
}
