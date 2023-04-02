use crate::server::hdresser::Hairdresser;
use crate::server::login::LoginData;
use crate::utils::dbstuff::{DatabaseQuery, DB_PATH};

pub fn get_hairdressers(hstyle: &str) -> Vec<Hairdresser> {
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
    hdressers
}

pub fn create_hairdresser(hd_name: &str, hd_email: &str) {
    let connection = sqlite::open(DB_PATH).unwrap();
    let query = DatabaseQuery::create_hdresser(&hd_email, &hd_name);
    connection.execute(query).unwrap();
}

pub fn get_picture_links(hdresser_id: i64, hstyle: &str) -> Vec<String> {
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
    pictures
}

impl LoginData {
    pub fn validation(&self) -> Result<String, String> {
        let existance = self.exist();
        if existance != -1 {
            let res = self.chech_password(existance);
            if !res.is_empty() {
                return Ok(res);
            }
        }
        Err("Your password is incorrect or this account doesn't exist".to_string())
    }

    fn exist(&self) -> i64 {
        let connection = sqlite::open(DB_PATH).unwrap();

        let query = DatabaseQuery::is_email_exist(&self.username);
        let mut statement = connection.prepare(&query).unwrap();
        let mut res = -1;
        while let Ok(sqlite::State::Row) = statement.next() {
            res = statement.read::<i64, _>("id").unwrap();
        }
        res
    }

    fn chech_password(&self, id: i64) -> String {
        let connection = sqlite::open(DB_PATH).unwrap();

        let query = DatabaseQuery::get_password(id, &self.password);
        let mut statement = connection.prepare(&query).unwrap();
        let mut res = String::new();
        while let Ok(sqlite::State::Row) = statement.next() {
            res = statement.read::<String, _>("name").unwrap();
        }
        res
    }
}
