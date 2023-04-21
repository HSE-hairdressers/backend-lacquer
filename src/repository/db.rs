use crate::server::hdresser::{Hairdresser, HairdresserIdentity};
use crate::server::login::LoginData;
use crate::server::reg::RegistrationData;
use crate::server::response::RegistrationResponse;
use crate::utils::dbstuff::{DatabaseQuery, DB_PATH};

use log::{debug, info};

pub fn get_hairdressers(hstyle: &str) -> Vec<Hairdresser> {
    info!(target: "repository/db/get-hairdressers", "Collecting hairdressers who has '{hstyle}' hairstyle.");
    let mut hdressers: Vec<Hairdresser> = vec![];

    let connection = sqlite::open(DB_PATH).unwrap();
    let query = DatabaseQuery::get_hdressers_by_hstyle(hstyle);
    let mut statement = connection.prepare(&query.0).unwrap();

    while let Ok(sqlite::State::Row) = statement.next() {
        let cur_data = (
            statement.read::<i64, _>(query.1 .0.as_str()).unwrap(),
            statement.read::<String, _>(query.1 .1.as_str()).unwrap(),
            statement.read::<String, _>(query.1 .2.as_str()).unwrap(),
        );
        hdressers.push(Hairdresser::new(
            cur_data.0,
            cur_data.1,
            cur_data.2,
            "No Phone".to_string(),
            "No address".to_string(),
            "No company".to_string(),
        ));
    }
    debug!(target: "repository/db/get-hairdresser", "{:?}", hdressers);
    hdressers
}

pub fn get_hairdresser(hd_id: i64) -> Hairdresser {
    info!(target: "repository/db/get-hairdresser", "Getting hairdresser with id:'{hd_id}'.");
    let connection = sqlite::open(DB_PATH).unwrap();
    let query = DatabaseQuery::get_hdressers_by_id(hd_id);
    let mut statement = connection.prepare(&query).unwrap();

    let mut hdresser = Hairdresser::with_id(hd_id);
    if let Ok(sqlite::State::Row) = statement.next() {
        hdresser.set_email(&statement.read::<String, _>("email").unwrap());
        hdresser.set_name(&statement.read::<String, _>("name").unwrap());
        hdresser.set_num(&statement.read::<String, _>("number").unwrap());
        hdresser.set_address(&statement.read::<String, _>("address").unwrap());
        hdresser.set_company(&statement.read::<String, _>("company").unwrap());
    }
    debug!(target: "repository/db/get-hairdresser", "{:?}", hdresser);
    hdresser
}

pub fn get_picture_links(hdresser_id: i64, hstyle: &str) -> Vec<String> {
    info!(target: "repository/db/get-pic-url", "Collecting picture urls.");
    let connection = sqlite::open(DB_PATH).unwrap();
    let query = DatabaseQuery::get_picture_urls(hdresser_id, hstyle);
    connection.execute(&query.0).unwrap();

    let mut statement = connection.prepare(&query.0).unwrap();

    let mut pictures: Vec<String> = vec![];
    while let Ok(sqlite::State::Row) = statement.next() {
        pictures.push(format!(
            "http://79.137.206.63:8000/{}",
            statement.read::<String, _>(query.1.as_str()).unwrap()
        ));
    }
    debug!(target: "repository/db/get-pic-url", "{:#?}", pictures);
    pictures
}

impl LoginData {
    pub fn validation(&self) -> Result<HairdresserIdentity, HairdresserIdentity> {
        info!(target: "repository/db/validation", "Starting validation.");
        let existance = self.exist();
        if existance != -1 {
            info!(target: "repository/db/validation", "User exists!");
            let res = self.check_password(existance);
            if !res.name.is_empty() {
                info!(target: "validation", "Successful login!");
                return Ok(res);
            } else {
                info!(target: "validation", "Incorrect password!");
            }
        } else {
            info!(target: "validation", "User does not exist!");
        }
        Err(HairdresserIdentity::new(
            -1,
            "Your password is incorrect or this account doesn't exist".to_string(),
        ))
    }

    fn exist(&self) -> i64 {
        info!(target: "validation", "Checking if user in database.");
        let connection = sqlite::open(DB_PATH).unwrap();

        let query = DatabaseQuery::is_email_exist(&self.username);
        let mut statement = connection.prepare(&query).unwrap();
        let mut res = -1;
        while let Ok(sqlite::State::Row) = statement.next() {
            res = statement.read::<i64, _>("id").unwrap();
        }
        debug!(target: "validation", "{:?}", res);
        res
    }

    fn check_password(&self, id: i64) -> HairdresserIdentity {
        info!(target: "validation", "Checking user password.");
        let connection = sqlite::open(DB_PATH).unwrap();

        let query = DatabaseQuery::get_password(id, &self.password);
        let mut statement = connection.prepare(&query).unwrap();
        let mut id: i64 = -1;
        let mut name = String::new();
        while let Ok(sqlite::State::Row) = statement.next() {
            id = statement.read::<i64, _>("id").unwrap();
            name = statement.read::<String, _>("name").unwrap();
        }
        debug!(target: "validation", "id: {:?} name: {:?}", id, name);
        HairdresserIdentity::new(id, name)
    }
}

impl RegistrationData {
    pub fn register(&self) -> RegistrationResponse {
        info!(target: "registration", "Starting registration.");
        match self.exist() {
            Err(_) => {
                info!(target: "registration", "Starting registration.");
                let connection = sqlite::open(DB_PATH).unwrap();

                let query = DatabaseQuery::add_user_to_db(
                    &self.username,
                    &self.name,
                    &self.phone,
                    &self.address,
                    &self.company,
                );
                connection.execute(query).unwrap();
                info!(target: "registration", "The user added to the database!");

                let id = self.exist().unwrap();
                let query = DatabaseQuery::change_password(id, &self.password);
                connection.execute(query).unwrap();
                info!(target: "registration", "The user's password added!");

                info!(target: "registration", "Successful registration!");
                RegistrationResponse::new("Ok")
            }
            Ok(_) => {
                info!(target: "registration", "User already exists!");
                RegistrationResponse::new("Failed")
            }
        }
    }

    fn exist(&self) -> Result<i64, &str> {
        info!(target: "registration", "Checking if user in database.");
        let connection = sqlite::open(DB_PATH).unwrap();

        let query = DatabaseQuery::is_email_exist(&self.username);
        let mut statement = connection.prepare(&query).unwrap();
        let mut res = -1;
        while let Ok(sqlite::State::Row) = statement.next() {
            res = statement.read::<i64, _>("id").unwrap();
        }
        debug!(target: "registration", "{:?}", res);
        if res == -1 {
            Err("User doesn't exist")
        } else {
            Ok(res)
        }
    }
}
