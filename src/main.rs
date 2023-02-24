pub mod utils;

use actix_multipart::Multipart;
use actix_web::{
    get, http::header::ContentType, post, web, App, Error, HttpResponse, HttpServer, Responder,
    Result,
};
use futures_util::StreamExt as _;
use serde::Serialize;
use std::io::Write;
use utils::ipstuff::IpAndPort;
use uuid::Uuid;

#[derive(Serialize)]
struct SystemInfo {
    version: String,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello World!".to_string()
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name} !")
}

#[get("/system/info")]
async fn sys_info() -> impl Responder {
    let mut resp: Vec<SystemInfo> = Vec::new();

    resp.push(SystemInfo {
        version: "0.0.1".to_string(),
    });

    return web::Json(resp);
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
            file = web::block(move || {
                file.write_all(&chunk.ok().unwrap_or_default())
                    .map(|_| file)
            })
            .await??
        }
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Ok")
        .into())
}

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
