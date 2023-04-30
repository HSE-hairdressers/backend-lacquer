use crate::repository::db;
use crate::server::{
    login::LoginData, photo::Photo, reg::RegistrationData, response::*, sysinfo::SystemInfo,
};
use actix_multipart::Multipart;
use actix_web::{
    get, http::header::ContentType, patch, post, web, Error, HttpResponse, Responder, Result,
};
use futures_util::StreamExt as _;
use log::{debug, info, warn};
use serde_json::Value;
use uuid::Uuid;

use std::io::Write;
use std::path::PathBuf;

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
    web::Json(info)
}

#[get("/hairdresser/info/{hd_id}")]
pub async fn get_hairdresser_info(hd_id: web::Path<i64>) -> impl Responder {
    debug!("{:?}", hd_id);
    let id = hd_id.into_inner();
    let response = db::get_hairdresser(id);
    web::Json(response)
}

#[patch("/hairdresser/edit/{hd_id}")]
pub async fn edit_hairdresser_info(
    hd_id: web::Path<i64>,
    to_edit: web::Json<Value>,
) -> HttpResponse {
    debug!("Got new editing request {:?} {:?}", hd_id, to_edit);
    let mut hairdresser = db::get_hairdresser(hd_id.into_inner());
    debug!("Data before editing: {:?}", hairdresser);
    if let Some(data) = to_edit.as_object() {
        for (k, v) in data {
            match k.as_str() {
                "email" => {
                    debug!("Email edited {:?}", v);
                    hairdresser.set_email(v.as_str().unwrap());
                }
                "name" => {
                    debug!("Name edited {:?}", v);
                    hairdresser.set_name(v.as_str().unwrap());
                }
                "num" => {
                    debug!("Phone number edited {:?}", v);
                    hairdresser.set_num(v.as_str().unwrap());
                }
                "addr" => {
                    debug!("Address edited {:?}", v);
                    hairdresser.set_address(v.as_str().unwrap());
                }
                "company" => {
                    debug!("Company edited {:?}", v);
                    hairdresser.set_company(v.as_str().unwrap());
                }
                &_ => {
                    debug!("Unknown field {:?} {:?}", k, v);
                }
            }
        }
    }
    debug!("Data after editing: {:?}", hairdresser);
    db::edit_hairdresser(hairdresser);
    HttpResponse::Ok().finish()
}

#[post("auth/login")]
pub async fn login(login_data: web::Json<LoginData>) -> Result<HttpResponse, Error> {
    info!("Login attempt received!");
    debug!("{:?}", login_data);
    let response = match login_data.validation() {
        Ok(i) => {
            info!("Login success.");
            LoginResponse::new("Ok", i)
        }
        Err(e) => {
            warn!("Wrong password or email!");
            LoginResponse::new("Error", e)
        }
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
        .into())
}

#[post("auth/registration")]
pub async fn registration(reg_data: web::Json<RegistrationData>) -> Result<HttpResponse, Error> {
    /*
     * "username"     : str,
     * "name"         : str,
     * "phone"        : str,
     * "address"      : str,
     * "company"      : str,
     * "password"     : str,
     * "verification" : str,
     * */
    info!("Registration attempt received!");
    debug!("{:?}", reg_data);

    let response = reg_data.register();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
        .into())
}

#[post("/img")]
pub async fn img(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut filename = String::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_disposition = field.content_disposition();

        let filename_part = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        filename.push_str(&filename_part);

        let _ = std::fs::create_dir("./tmp");
        let filepath = format!("./tmp/{}", &filename);
        let mut file = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.next().await {
            file = web::block(move || {
                file.write_all(&chunk.ok().unwrap_or_default())
                    .map(|_| file)
            })
            .await??
        }
    }

    info!("New photo received!");

    let filepath = PathBuf::from(format!("./tmp/{}", &filename));

    match recognize_hairstyle(&filepath).await {
        Ok(Some(hstyle)) => {
            info!("Photo was recognized!",);
            let hairdressers = db::get_hairdressers(&hstyle);
            let data = hairdressers
                .into_iter()
                .map(|hdresser| {
                    DataResponse::new(
                        hdresser.clone(),
                        Photo::from_vec(db::get_picture_links(hdresser.get_id(), &hstyle)),
                    )
                })
                .collect();
            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(UserImageResponse::with_data(data))
                .into())
        }
        _ => {
            info!("Photo wasn't recognized!");
            Ok(HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .json(UserImageResponse::new())
                .into())
        }
    }
}

async fn recognize_hairstyle(photo_p: &PathBuf) -> Result<Option<String>, reqwest::Error> {
    let data = std::fs::read(photo_p).unwrap();

    info!("Photo opened successfully!");
    debug!("{:?}", photo_p);

    info!("Photo sent to the classifier.");
    let client = reqwest::Client::new();
    let res = client
        .post("http://hairclassificator-web-1:8022/api/test")
        .body(data)
        .send()
        .await?;
    let hairstyle = res.json::<HairClassifierResponse>().await?;
    debug!("{:?}", hairstyle);
    Ok(hairstyle.get_result())
}

#[post("/hairdresser/upload")]
pub async fn upload_image(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut filepath = PathBuf::new();
    let mut id_value: i64 = 0;
    debug!("received photo from hairdresser");
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();
        match content_type.get_name().unwrap_or("") {
            "id" => {
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk?;
                    bytes.extend_from_slice(&chunk);
                }
                id_value = String::from_utf8(bytes).unwrap().parse().unwrap();
            }
            _ => {
                let filename = content_type.get_filename().unwrap_or("");
                let _ = std::fs::create_dir_all("./tmp/test");

                filepath = PathBuf::from(format!("./tmp/test/{}", filename));
                debug!("try to save file in {:?}", &filepath);
                let _fp = filepath.clone();
                let mut file = web::block(|| std::fs::File::create(_fp)).await??;

                while let Some(chunk) = field.next().await {
                    file = web::block(move || {
                        file.write_all(&chunk.ok().unwrap_or_default())
                            .map(|_| file)
                    })
                    .await??
                }
                debug!("{:?} saved", filepath);
            }
        };
    }
    match recognize_hairstyle(&filepath).await {
        Ok(Some(hstyle)) => {
            #[derive(serde::Serialize)]
            struct UploadImageRequest {
                photo_bin: Vec<u8>,
                folder_name: String,
                secret_pass: String,
            }
            let data = std::fs::read(&filepath).unwrap();
            debug!("file {:?} opened", filepath);

            info!("Making response for adding photo to the db");
            let response = reqwest::Client::new()
                .post("http://79.137.206.63:8000")
                .json(&UploadImageRequest {
                    photo_bin: data,
                    folder_name: format!("{}/{}", id_value, &hstyle),
                    secret_pass: "wearehairdressers".to_string(),
                })
                .send()
                .await
                .unwrap();
            let resp = response.json::<serde_json::Value>().await.unwrap();
            debug!("{:?}", resp);
            if let Some(message) = resp.get("message").and_then(|m| m.as_str()) {
                db::add_photo_to_db(id_value, message, &hstyle);
            }
            Ok(HttpResponse::Ok().into())
        }
        _ => Ok(HttpResponse::BadRequest().into()),
    }
}
