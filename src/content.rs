use crate::repository::db;
use crate::server::login::LoginData;
use crate::server::reg::RegistrationData;
use crate::server::{
    hdresser::Hairdresser,
    photo::Photo,
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
    info!("Login attempt received!");
    debug!("{:?}", login_data);
    let response = match login_data.validation() {
        Ok(i) => {
            info!("Login success.");
            LoginResponse::new("Ok", &i)
        }
        Err(e) => {
            warn!("Wrong password or email!");
            LoginResponse::new("Error", &e)
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

    info!("New photo received!");
    let filepath = format!("./tmp/{filename}");
    let data = std::fs::read(filepath.clone()).unwrap();
    info!("Photo opened successfully!");
    debug!("{:?}", filepath);

    info!("Photo sent to the classifier.");
    let client = reqwest::Client::new();
    let res = client
        .post("http://hairclassificator-web-1:8022/api/test")
        .body(data)
        .send()
        .await
        .unwrap();
    let hairstyle = res.json::<HairClassifierResponse>().await.unwrap();

    if let Some(hstyle) = hairstyle.get_result() {
        info!("Photo classified!");
        debug!("{:?}", hstyle);
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
        info!(
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
