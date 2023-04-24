use crate::repository::db;
use crate::server::hdresser::HairdresserId;
use crate::server::{
    login::LoginData,
    photo::Photo,
    reg::RegistrationData,
    response::{DataResponse, HairClassifierResponse, LoginResponse, UserImageResponse},
    sysinfo::SystemInfo,
};
use actix_multipart::Multipart;
use actix_web::{
    get, http::header::ContentType, post, web, Error, HttpResponse, Responder, Result,
};
use futures_util::StreamExt as _;
use log::{debug, info, warn};
use serde_json::json;
use uuid::Uuid;

use std::io::Write;

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

#[post("auth/login")]
pub async fn login(login_data: web::Json<LoginData>) -> Result<HttpResponse, Error> {
    info!(target: "content/login", "Login attempt received!");
    debug!(target: "content/login", "{:?}", login_data);
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

#[post("hairdresser/info")]
pub async fn get_hairdresser_info(hd_id: web::Json<HairdresserId>) -> Result<HttpResponse, Error> {
    debug!(target: "content/get_hairdresser_info", "{:?}", hd_id);
    let id = hd_id.get_id();
    let response = db::get_hairdresser(id);
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
    info!(target: "content/registration", "Registration attempt received!");
    debug!(target: "content/registration", "{:?}", reg_data);

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

        filename.push_str(
            content_disposition
                .get_filename()
                .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize)
                .as_str(),
        );

        let _ = std::fs::create_dir("./tmp");
        let filepath = format!("./tmp/{filename}");

        let mut file = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.next().await {
            file = web::block(move || {
                file.write_all(&chunk.ok().unwrap_or_default())
                    .map(|_| file)
            })
            .await??
        }
    }

    info!(target: "content/img", "New photo received!");
    let hstyle_resp = recognize_hairstyle(&format!("./tmp/{filename}")).await;

    let response = if let Some(hstyle) = hstyle_resp {
        let mut response = UserImageResponse::new("Ok");
        for hdresser in db::get_hairdressers(&hstyle) {
            let images = Photo::from_vec(db::get_picture_links(hdresser.get_id(), &hstyle));
            let data_res = DataResponse::new(hdresser, images);
            response.add_data(data_res);
        }
        response
    } else {
        info!(target: "content/img",
            "Photo wasn't recognized.",
        );
        UserImageResponse::new("Error")
    };
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
        .into())
}

async fn recognize_hairstyle(photo_p: &str) -> Option<String> {
    let filepath = photo_p.to_string();
    let data = std::fs::read(filepath.clone()).unwrap();
    info!(target: "content/recognize_hairstyle", "Photo opened successfully!");
    debug!(target: "content/recognize_hairstyle", "{:?}", filepath);

    info!(target: "content/recognize_hairstyle", "Photo sent to the classifier.");
    let client = reqwest::Client::new();
    let res = client
        .post("http://hairclassificator-web-1:8022/api/test")
        .body(data)
        .send()
        .await
        .unwrap();
    let hairstyle = res.json::<HairClassifierResponse>().await.unwrap();
    debug!(target: "content/recognize_hairstyle","{:?}", hairstyle);
    hairstyle.get_result()
}

#[post("/hairdresser/upload")]
pub async fn upload_image(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();
        let mut id_value = String::new();

        let filepath = match content_type.get_name() {
            Some("id") => {
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk.unwrap();
                    bytes.extend_from_slice(&chunk);
                }
                id_value = String::from_utf8(bytes).unwrap();
                // println!("id: {}", id_value);
                debug!(target: "content/upload_image","received photo from hairdresser with id: {:?}", id_value);
                None
            }
            _ => {
                let filename = content_type.get_filename().unwrap_or("");
                debug!(target: "content/upload_image","received photo from hairdresser with name {:?}", filename);

                if let Ok(()) = std::fs::create_dir("./tmp") {
                    debug!(target: "content/upload_image","folder ./tmp successfully created");
                }
                if let Ok(()) = std::fs::create_dir("./tmp/test") {
                    debug!(target: "content/upload_image","folder ./tmp/test successfully created");
                }

                let filepath = format!("./tmp/test/{filename}");
                debug!(target: "content/upload_image","try to save file in {:?}", &filepath);
                let _filepath = filepath.clone();
                let mut file = web::block(|| std::fs::File::create(_filepath)).await??;

                while let Some(chunk) = field.next().await {
                    file = web::block(move || {
                        file.write_all(&chunk.ok().unwrap_or_default())
                            .map(|_| file)
                    })
                    .await??
                }
                debug!(target: "content/upload_image","{} saved", filepath);
                Some(filepath)
            }
        };

        if let Some(filepath) = filepath {
            let hstyle_resp = recognize_hairstyle(&filepath).await;

            if let Some(hstyle) = hstyle_resp {
                #[derive(serde::Serialize)]
                struct UploadImageRequest {
                    photo_bin: Vec<u8>,
                    folder_name: String,
                    secret_pass: String,
                }


                let data = std::fs::read(filepath.clone()).unwrap();
                let payload = json!({
                    "photo_bin" : data,
                    "folder_name" : format!("{}/{}", id_value, hstyle),
                    "secret_pass" : "wearehairdressers".to_string(),}
            );
                let response = reqwest::Client::new()
                    .post("http://79.137.206.63:8000")
                    .json(&payload)
                    .send()
                    .await
                    .unwrap();
                info!(target: "content/upload_image", "{}", response.text().await.unwrap());
            } else {
                info!(target: "content/upload_image",
                    "Photo wasn't recognized"
                );
            }
        }
    }
    Ok(HttpResponse::Ok().into())
}
