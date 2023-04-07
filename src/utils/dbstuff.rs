pub const DB_PATH: &str = "./hairdressers.db";
pub const HAIRSTYLES: &'static [&str] = &[
    "afro",
    "bob",
    "buzz_cut",
    "caesar_cut",
    "crew",
    "dreadlocks",
    "ivy_league",
    "pixie",
    "pixie_bob",
    "pixie_long",
    "twists",
    "buzz",
];

#[derive(Debug)]
pub struct DatabaseQuery;

impl DatabaseQuery {
    pub fn get_hdresser_id(hdresser_email: &str) -> (String, String) {
        let index = "id";
        let query = format!(
            "SELECT hairdressers.id as {index} WHERE hairdressers.email = '{hdresser_email}'"
        );
        (query.to_owned(), index.to_string())
    }

    pub fn create_hdresser(hd_email: &str, hd_password: &str) -> String {
        let _query = format!("INSERT INTO hairdressers VALUES ('{hd_email}', '{hd_password}');");
        // query.to_owned();
        todo!("add hdresser by email and password")
    }

    pub fn get_hdressers_by_hstyle(hstyle: &str) -> (String, (String, String, String)) {
        let index = (
            "hd_id".to_string(),
            "hd_email".to_string(),
            "hd_name".to_string(),
        );
        let query = format!(
            "
                SELECT DISTINCT hairdressers.id as {}, hairdressers.email as {}, hairdressers.name as {}
                FROM style_to_dresser
                JOIN hairdressers ON hairdresser_id = hairdressers.id
                JOIN hairstyles ON hairstyle_name = hairstyles.name
                WHERE hairstyles.name = '{hstyle}'
            ",
            index.0, index.1, index.2
        );
        (query.to_owned(), index)
    }

    pub fn get_picture_urls(hd_id: i64, hstyle: &str) -> (String, String) {
        let index = "urls";
        let query = format!(
            "
                SELECT img_url as {index}
                FROM style_to_dresser
                JOIN hairstyles ON hairstyle_name = hairstyles.name
                JOIN hairdressers ON hairdresser_id = hairdressers.id
                WHERE hairdresser_id = '{hd_id}' AND hairstyle_name ='{hstyle}'
            "
        );
        (query.to_owned(), index.to_string())
    }

    pub fn is_email_exist(email: &str) -> String {
        let query = format!(
            "
                SELECT hairdressers.id as id
                FROM dresser_login_info
                JOIN hairdressers ON hairdressers.id = dresser_login_info.id
                WHERE hairdressers.email = '{email}'
            "
        );
        query.to_owned()
    }

    pub fn get_password(hd_id: i64, pass: &str) -> String {
        let query = format!(
            "
                SELECT hairdressers.name as name
                FROM hairdressers
                JOIN dresser_login_info ON hairdressers.id = dresser_login_info.id
                WHERE hairdressers.id = {hd_id} AND password = '{pass}'
            "
        );
        query.to_owned()
    }

    pub fn add_user_to_db(email: &str, name: &str, num: &str, addr: &str, com: &str) -> String {
        let query = format!("INSERT INTO hairdressers (email, name, number, address, company) VALUES ('{email}', '{name}', '{num}', '{addr}', '{com}');");
        query.to_owned()
    }

    pub fn change_password(hd_id: i64, pass: &str) -> String {
        let query = format!("INSERT INTO dresser_login_info VALUES ({hd_id}, '{pass}');");
        query.to_owned()
    }
}
