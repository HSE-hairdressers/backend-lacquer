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
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use std::io::Write;
use std::path::PathBuf;

/// This service returns information about the version of current running server.
#[get("/system/info")]
pub async fn sys_info() -> impl Responder {
    let mut info = SystemInfo::default();

    /// Parse Cargo.toml file.
    /// Extract name of the package and the version.
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

/// This service returns information about the hairdresser by its id.
#[get("/hairdresser/info/{hd_id}")]
pub async fn get_hairdresser_info(hd_id: web::Path<i64>) -> impl Responder {
    debug!("{:?}", hd_id);

    let id = hd_id.into_inner();
    /// Use `db` module in order to communicate with database.
    let response = db::get_hairdresser(id);
    web::Json(response)
}

#[get("/hairdresser/images/{hd_id}")]
pub async fn get_hairdresser_images(hd_id: web::Path<i64>) -> impl Responder {
    debug!("{:?}", hd_id);

    let id = hd_id.into_inner();

    #[derive(Debug, Serialize, Deserialize)]
    struct HairdresserImages {
        images: Vec<Photo>,
    };
    let response = HairdresserImages {
        images: Photo::from_vec(db::get_images(id)),
    };

    web::Json(response)
}

/// This service receive hairdresser's id and values in JSON that needs to be changed.
#[patch("/hairdresser/edit/{hd_id}")]
pub async fn edit_hairdresser_info(
    hd_id: web::Path<i64>,
    to_edit: web::Json<Value>,
) -> HttpResponse {
    debug!("Got new editing request {:?} {:?}", hd_id, to_edit);
    /// Use `db` module in order to get old hairdresser's information
    /// from the database.
    let mut hairdresser = db::get_hairdresser(hd_id.into_inner());
    debug!("Data before editing: {:?}", hairdresser);

    /// Catch changed information from the Json object.
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

    /// Use `db` module in order to update hairdresser's information in the database.
    db::edit_hairdresser(hairdresser);
    HttpResponse::Ok().finish()
}

/// This service receives login attempt and then check if user's password and login are correct.
#[post("auth/login")]
pub async fn login(login_data: web::Json<LoginData>) -> Result<HttpResponse, Error> {
    info!("Login attempt received!");
    debug!("{:?}", login_data);

    /// Use method `validation` implemented in `db` module
    /// that helps to communicate with the database;
    let response = match login_data.validation() {
        Ok(i) => {
            info!("Login success.");
            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(i)
                .into())
        }
        Err(e) => {
            warn!("Wrong password or email!");
            Ok(HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .json(e)
                .into())
        }
    };
    response
}

/// This service receives registration attempt and then check if user already exists.
/// If no then try to add new user with given username (email) and password.
#[post("auth/registration")]
pub async fn registration(reg_data: web::Json<RegistrationData>) -> Result<HttpResponse, Error> {
    info!("Registration attempt received!");
    debug!("{:?}", reg_data);

    let response = json!({"result" : reg_data.register()});
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
        .into())
}

/// This service receives multipart a single picture.
/// Then sends this picture to the AI.
/// If service gets successfull response, then it find all hairdressers that have photos with
/// current hairstyle.
#[post("/img")]
pub async fn img(payload: Multipart) -> Result<HttpResponse, Error> {
    let (_, filepath) = utilize_multipart(payload).await.unwrap();
    info!("New photo received!");

    let response = match recognize_hairstyle(&filepath).await {
        Ok(Some(hstyle)) => {
            info!("Photo was recognized!",);

            let hairdressers = db::get_hairdressers(&hstyle);
            let data = hairdressers
                .into_iter()
                .map(|hd| {
                    HairdresserData::new(
                        hd.clone(),
                        Photo::from_vec(db::get_picture_links(hd.get_id(), &hstyle)),
                    )
                })
                .collect();
            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(HairdresserResponseBuilder::with_data(data).build())
                .into())
        }
        _ => {
            info!("Photo wasn't recognized!");

            Ok(HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .json(HairdresserResponse::builder().build())
                .into())
        }
    };
    debug!("Try to delete file {:?}", filepath);
    let _ = std::fs::remove_file(filepath);
    debug!("Deleted successfully!");
    response
}

/// This service receives multipart with "id" and "img" fields.
/// Then saves "img" photo to the database in hairdresser's profile by its "id".
#[post("/hairdresser/upload")]
pub async fn upload_image(payload: Multipart) -> Result<HttpResponse, Error> {
    ///
    /// # Arguments
    ///
    /// *`payload` - A Multipart object with "id" and "img" fields.
    ///
    let (id, filepath) = utilize_multipart(payload).await.unwrap();
    let id_value = id.unwrap();

    let response = match recognize_hairstyle(&filepath).await {
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

            /// Make request to web-service that helps to save hairdresser's photo.
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
    };
    debug!("Try to delete file {:?}", filepath);
    let _ = std::fs::remove_file(filepath);
    debug!("Deleted successfully!");
    response
}

/// Function that gets Multipart object and then try to extract "id" and "img" fields from them.
async fn utilize_multipart(mut payload: Multipart) -> Result<(Option<i64>, PathBuf), Error> {
    ///
    /// # Arguments
    ///
    /// *`payload` - A Multipart object that needs to be extracted.
    ///
    let mut id = None;
    let mut filepath = PathBuf::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();
        match content_disposition.get_name().unwrap_or("") {
            "id" => {
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk?;
                    bytes.extend_from_slice(&chunk);
                }
                id = Some(String::from_utf8(bytes).unwrap().parse().unwrap());
            }
            _ => {
                let filename = content_disposition
                    .get_filename()
                    .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
                let _ = std::fs::create_dir_all("./tmp");

                filepath = PathBuf::from(format!("./tmp/{}", filename));

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
    Ok((id, filepath))
}

/// Function that receives path to the photo and then tries to recognize hairstyle.
async fn recognize_hairstyle(photo_p: &PathBuf) -> Result<Option<String>, reqwest::Error> {
    /// Function opens photo by its path.
    /// Then sends it to the AI that will try to recognize a hairstyle.
    /// If recognized successfull then send result back.
    ///
    /// # Arguments
    ///
    /// *`photo_p` - A path to the photo that needs to be recognized on local machine.
    ///
    debug!("try to open {:?}", photo_p);
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
