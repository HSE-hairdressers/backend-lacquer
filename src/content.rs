use crate::server::{
    hdresser::Hairdresser,
    photo::Photo,
    response::{DataResponse, HairClassifierResponse, UserImageResponse},
    sysinfo::SystemInfo,
};
use actix_multipart::Multipart;
use actix_web::{
    get, http::header::ContentType, post, web, Error, HttpResponse, Responder, Result,
};
use futures_util::StreamExt as _;
use uuid::Uuid;

use std::io::Write;

#[get("/hello")]
pub async fn hello() -> impl Responder {
    "Hello World!".to_string()
}

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name} !")
}

#[get("/system/info")]
pub async fn sys_info() -> impl Responder {
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
pub async fn img(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut filename = String::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_disposition = field.content_disposition();

        filename.push_str(
            content_disposition
                .get_filename()
                .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize)
                .as_str(),
        );

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

    println!("[ ------------------- ]");
    println!("[ NEW PHOTO RECEIVED! ]");
    println!("[ ------------------- ]");

    println!("[INFO] Open photo");
    let filepath = format!("./tmp/{filename}");
    let data = std::fs::read(filepath).unwrap();

    println!("[INFO] Send photo to the classifier");
    let client = reqwest::Client::new();
    let res = client
        .post("http://hairclassificator-web-1:8022/api/test")
        .body(data)
        .send()
        .await
        .unwrap();
    let hairstyle = res.json::<HairClassifierResponse>().await.unwrap();

    println!("[INFO] Got hairstyle {:#?}", hairstyle);

    let f_path = format!("./onlyfaces/{}/", hairstyle.result);
    println!("[INFO] Try to open dir {:#?}", f_path);
    let paths = std::fs::read_dir(f_path.to_string()).unwrap();

    println!("[INFO] Create hairdresser");
    let hairdresser = Hairdresser::new(
        "Khadiev Edem".to_string(),
        "+7 999 123 45 67".to_string(),
        "NN, Test st., 100100".to_string(),
        "HSE-hairdressers".to_string(),
    );

    let mut photos: Vec<Photo> = Vec::new();
    let mut i = 0;
    println!("[INFO] Add all photos to a photos");
    for path in paths.into_iter() {
        i += 1;
        let photo = Photo::new(format!("{i} photo"), format!("http://79.137.206.63:8000/{}/{}", hairstyle.result.replace(" ", "_"), path.unwrap().file_name().into_string().unwrap()));
        photos.push(photo);
    }

    let data_res = DataResponse::new(hairdresser, photos);
    
    let result = match hairstyle.result.as_str() {
        "0" => "Error",
        _ => "Ok",
    };
    let mut response = UserImageResponse::new(result);
    response.add_data(data_res);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
        .into())
}
