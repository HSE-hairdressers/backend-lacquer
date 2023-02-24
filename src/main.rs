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

#[derive(Serialize, Default)]
struct SystemInfo {
    name: String,
    version: String,
}

#[derive(Serialize)]
struct Photo {
    name: String,
    binary: Vec<u8>,
}

impl Photo {
    fn new(name: String, binary: Vec<u8>) -> Self {
        Photo {
            name: (name),
            binary: (binary),
        }
    }
}

#[derive(Serialize)]
struct Hairdresser {
    name: String,
    num: String,
    addr: String,
    company: String,
}

impl Hairdresser {
    fn new(name: String, phone_number: String, address: String, company: String) -> Self {
        Hairdresser {
            name: (name),
            num: (phone_number),
            addr: (address),
            company: (company),
        }
    }
}

#[derive(Serialize)]
struct UserImageResponse {
    hairdresser: Hairdresser,
    images: Vec<Photo>,
    result: String,
}

impl UserImageResponse {
    fn new(h_name: Hairdresser, images: Vec<Photo>) -> Self {
        UserImageResponse {
            hairdresser: (h_name),
            images: (images),
            result: ("Ok".to_string()),
        }
    }
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
    let mut info = SystemInfo::default();

    for line in std::fs::read_to_string("./Cargo.toml").unwrap().split("\n") {
        if line.contains("name =") {
            info.name = line.replace("name = ", "").replace("\"", "");
        } else if line.contains("version = ") {
            info.version = line.replace("version = ", "").replace("\"", "");
            break;
        }
    }
    return web::Json(info);
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

    let hairdresser = Hairdresser::new(
        "Khadiev Edem".to_string(),
        "+7 999 123 45 67".to_string(),
        "NN, Test st., 100100".to_string(),
        "HSE-hairdressers".to_string(),
    );

    let filename = "test.jpeg";
    let filepath = format!("./{filename}");
    let photo = Photo::new(
        filename.to_string(),
        web::block(|| std::fs::read(filepath)).await??,
    );

    let mut photos: Vec<Photo> = Vec::new();
    photos.push(photo);

    let response = UserImageResponse::new(hairdresser, photos);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
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
