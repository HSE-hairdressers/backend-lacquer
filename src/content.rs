use crate::repository::db;
use crate::server::hdresser::HairdresserId;
use crate::server::{
    hdresser::Hairdresser,
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
    let filepath = format!("./tmp/{filename}");
    let data = std::fs::read(filepath.clone()).unwrap();
    info!(target: "content/img", "Photo opened successfully!");
    debug!(target: "content/img", "{:?}", filepath);

    info!(target: "img", "Photo sent to the classifier.");
    let client = reqwest::Client::new();
    let res = client
        .post("http://hairclassificator-web-1:8022/api/test")
        .body(data)
        .send()
        .await
        .unwrap();
    let hairstyle = res.json::<HairClassifierResponse>().await.unwrap();

    if let Some(hstyle) = hairstyle.get_result() {
        info!(target: "content/img", "Photo classified!");
        debug!(target: "content/img","{:?}", hstyle);
        let mut response = UserImageResponse::new("Ok");
        let hdressers: Vec<Hairdresser> = db::get_hairdressers(&hstyle); // vector with hairdressers
        for hdresser in hdressers {
            let img_urls: Vec<String> = db::get_picture_links(hdresser.get_id(), &hstyle);
            let images: Vec<Photo> = Photo::from_vec(&img_urls);
            let data_res = DataResponse::new(hdresser, images);
            response.add_data(data_res);
        }
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(response)
            .into())
    } else {
        info!(target: "content/img",
            "Photo wasn't recognized with message: \"{}\"",
            hairstyle.message
        );
        let response = UserImageResponse::new("Error");
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(response)
            .into())
    }
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

                let data = std::fs::read(filepath.clone()).unwrap();
                info!(target:  "content/upload_image", "Photo opened successfully!");
                debug!(target: "content/upload_image", "{:?}", filepath);

                info!(target: "content/upload_image", "Photo sent to the classifier.");
                let client = reqwest::Client::new();
                let res = client
                    .post("http://hairclassificator-web-1:8022/api/test")
                    .body(data.clone())
                    .send()
                    .await
                    .unwrap();
                let hairstyle = res.json::<HairClassifierResponse>().await.unwrap();
                if let Some(hstyle) = hairstyle.get_result() {
                    info!(target:  "content/upload_image", "Photo classified!");
                    debug!(target: "content/upload_image","{:?}", hstyle);

                    #[derive(serde::Serialize)]
                    struct UploadImageRequest {
                        photo_bin: Vec<u8>,
                        folder_name: String,
                        secret_pass: String,
                    }
                    let _res = client
                        .post("http://79.137.206.63:8000")
                        .json(&UploadImageRequest {
                            photo_bin: data,
                            folder_name: format!("{}/{}", id_value, hstyle),
                            secret_pass: "wearehairdressers".to_string(),
                        })
                        .send()
                        .await
                        .unwrap();
                } else {
                    info!(target: "content/upload_image",
                        "Photo wasn't recognized"
                    );
                }
        }
    }
    Ok(HttpResponse::Ok().into())
}
