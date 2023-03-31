use crate::server::hdresser::Hairdresser;
use crate::utils::dbstuff::{DatabaseQuery, DB_PATH, HAIRSTYLES};

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
            "No email".to_string(),
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

pub fn init_db() {
    if !std::path::Path::new(&DB_PATH).exists() {
        let connection = sqlite::open(DB_PATH).unwrap();
        let query = DatabaseQuery::init_db();
        connection.execute(query).unwrap();

        for style in HAIRSTYLES {
            let ins_query = format!("INSERT INTO hairstyles VALUES ('{style}');");
            connection.execute(&ins_query).unwrap();
        }

        let baby_dressers: Vec<_> = vec![
            (
                DatabaseQuery::get_hdresser_id("khadiev.edem@gmail.com"),
                "Khadiev Edem",
            ),
            (
                DatabaseQuery::get_hdresser_id("max.md8@gmail.com"),
                "Dudarev Maxim",
            ),
            (
                DatabaseQuery::get_hdresser_id("ageev.maxim2003@gmail.com"),
                "Ageev Maxim",
            ),
        ];

        for query in baby_dressers {
            let mut statement = connection.prepare(&query.0 .0).unwrap();
            let mut baby_dr = String::new();
            while let Ok(sqlite::State::Row) = statement.next() {
                baby_dr = statement.read::<String, _>(query.0 .1.as_str()).unwrap();
            }
            for style in HAIRSTYLES {
                let ins_query = format!(
                    "
                    INSERT INTO style_to_dresser VALUES ({baby_dr}, '{}', '{style}');
                    ",
                    query.1
                );
                connection.execute(&ins_query).unwrap();
            }
        }
    }
}

pub fn get_picture_links(hdresser_id: i64, hstyle: &str) -> Vec<String> {
    let connection = sqlite::open(DB_PATH).unwrap();
    let query = DatabaseQuery::get_picture_urls(hdresser_id, hstyle);
    connection.execute(&query.0).unwrap();

    let mut statement = connection.prepare(&query.0).unwrap();

    let mut pictures: Vec<String> = vec![];
    while let Ok(sqlite::State::Row) = statement.next() {
        pictures.push(statement.read::<String, _>(query.1.as_str()).unwrap());
    }
    pictures
}
