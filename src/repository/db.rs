use crate::server::hdresser::{Hairdresser, HairdresserIdentity};
use crate::server::login::LoginData;
use crate::server::reg::RegistrationData;
use crate::utils::dbstuff::{DatabaseQuery, DB_PATH};

use log::{debug, info};

/// Returns a vector with hairdressers by given hairstyle.
///
/// # Arguments
///
/// * `hstyle` - A string that holds hairstyle to find hairdressers..
pub fn get_hairdressers(hstyle: &str) -> Vec<Hairdresser> {
    info!("Collecting hairdressers who has '{}' hairstyle.", hstyle);
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
    debug!("{:?}", hdressers);
    hdressers
}

/// Returns a hairdresser by given id.
///
/// # Arguments
///
/// * `id` - A number that holds hairdresser's id in database.
pub fn get_hairdresser(hd_id: i64) -> Hairdresser {
    info!("Getting hairdresser with id:'{}'.", hd_id);
    let connection = sqlite::open(DB_PATH).unwrap();
    let query = DatabaseQuery::get_hdressers_by_id(hd_id);
    let mut statement = connection.prepare(&query).unwrap();

    let mut hdresser = Hairdresser::with_id(hd_id);
    if let Ok(sqlite::State::Row) = statement.next() {
        hdresser.set_email(&statement.read::<String, _>("email").unwrap());
        hdresser.set_name(&statement.read::<String, _>("name").unwrap());
        if let Ok(num) = statement.read::<String, _>("number") {
            hdresser.set_num(&num);
        }
        if let Ok(addr) = statement.read::<String, _>("address") {
            hdresser.set_address(&addr);
        }
        if let Ok(company) = statement.read::<String, _>("company") {
            hdresser.set_company(&company);
        }
    }
    debug!("{:?}", hdresser);
    hdresser
}

/// Function that edits hairdresser's info.
///
/// # Arguments
///
/// * `hairdresser` - An object that represents a hairdresser.
pub fn edit_hairdresser(hairdresser: Hairdresser) {
    let connection = sqlite::open(DB_PATH).unwrap();
    let query = DatabaseQuery::edit_hairdresser_info(
        hairdresser.get_id(),
        hairdresser.get_email(),
        hairdresser.get_name(),
        hairdresser.get_number(),
        hairdresser.get_address(),
        hairdresser.get_company(),
    );
    connection.execute(&query).unwrap();
}

/// Returns a vector with photo URLs by given hairdresser's id and hairstyle.
///
/// # Arguments
///
/// * `id` - A number that holds hairdresser's id in database.
/// * `hstyle` - A string that holds hairstyle to find hairdressers..
pub fn get_picture_links(hdresser_id: i64, hstyle: &str) -> Vec<String> {
    info!("Collecting picture urls.");
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
    debug!("{:#?}", pictures);
    pictures
}

/// Function that uploads photo to the database by hairstyle and hairdresser's id.
///
/// # Arguments
///
/// * `hd_id` - A number that holds hairdresser's id in database.
/// * `photo_name` - A string that holds photo's name.
/// * `hstyle` - A string that holds hairstyle to find hairdressers..
pub fn add_photo_to_db(hd_id: i64, photo_name: &str, hstyle: &str) {
    debug!(
        "adding photo with name {} to hairdresser with id: {}",
        photo_name, hd_id
    );
    let connection = sqlite::open(DB_PATH).unwrap();
    let query = DatabaseQuery::add_photo_to_db(hd_id, photo_name, hstyle);
    connection.execute(query).unwrap();
    debug!("Photo {} successfully added!", photo_name);
}

impl LoginData {
    /// Method that checks correctness of hairdresser's username and password.
    /// Returns a hairdresser's identity.
    pub fn validation(&self) -> Result<HairdresserIdentity, HairdresserIdentity> {
        info!("Starting validation.");
        let existance = self.exist();
        if existance != -1 {
            info!("User exists!");
            let res = self.check_password(existance);
            if !res.name.is_empty() {
                info!("Successful login!");
                return Ok(res);
            } else {
                info!("Incorrect password!");
            }
        } else {
            info!("User does not exist!");
        }
        Err(HairdresserIdentity::new(
            -1,
            "Your password is incorrect or this account doesn't exist".to_string(),
        ))
    }

    /// Returns a hairdresser's id if hairdresser exists in database.
    fn exist(&self) -> i64 {
        info!("Checking if user in database.");
        let connection = sqlite::open(DB_PATH).unwrap();

        let query = DatabaseQuery::is_email_exist(&self.username);
        let mut statement = connection.prepare(&query).unwrap();
        let mut res = -1;
        while let Ok(sqlite::State::Row) = statement.next() {
            res = statement.read::<i64, _>("id").unwrap();
        }
        debug!("{:?}", res);
        res
    }

    /// Method that checks hairdresser's password correctness.
    fn check_password(&self, id: i64) -> HairdresserIdentity {
        info!("Checking user password.");
        let connection = sqlite::open(DB_PATH).unwrap();

        let query = DatabaseQuery::get_password(id, &self.password);
        let mut statement = connection.prepare(&query).unwrap();
        let mut id: i64 = -1;
        let mut name = String::new();
        while let Ok(sqlite::State::Row) = statement.next() {
            id = statement.read::<i64, _>("id").unwrap();
            name = statement.read::<String, _>("name").unwrap();
        }
        debug!("id: {:?} name: {:?}", id, name);
        HairdresserIdentity::new(id, name)
    }
}

impl RegistrationData {
    /// That adds hairdresser to the database
    pub fn register(&self) -> String {
        info!("Starting registration.");
        match self.exist() {
            Err(_) => {
                info!("Starting registration.");
                let connection = sqlite::open(DB_PATH).unwrap();

                let query = DatabaseQuery::add_user_to_db(
                    &self.username,
                    &self.name,
                    &self.phone,
                    &self.address,
                    &self.company,
                );
                connection.execute(query).unwrap();
                info!("The user added to the database!");

                let id = self.exist().unwrap();
                let query = DatabaseQuery::change_password(id, &self.password);
                connection.execute(query).unwrap();
                info!("The user's password added!");

                info!("Successful registration!");
                "Successful registration!".to_string()
            }
            Ok(_) => {
                info!("User already exists!");
                "Registration failed. User already exists!".to_string()
            }
        }
    }

    /// Returns a hairdresser's id if hairdresser exists in database.
    fn exist(&self) -> Result<i64, &str> {
        info!("Checking if user in database.");
        let connection = sqlite::open(DB_PATH).unwrap();

        let query = DatabaseQuery::is_email_exist(&self.username);
        let mut statement = connection.prepare(&query).unwrap();
        let mut res = -1;
        while let Ok(sqlite::State::Row) = statement.next() {
            res = statement.read::<i64, _>("id").unwrap();
        }
        debug!("{:?}", res);
        if res == -1 {
            Err("User doesn't exist")
        } else {
            Ok(res)
        }
    }
}
