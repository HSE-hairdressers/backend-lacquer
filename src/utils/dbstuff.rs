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

/// In order to communicate with database we use SQL queries.
#[derive(Debug)]
pub struct DatabaseQuery;

impl DatabaseQuery {
    /// Returns a query for getting hairdresser's id by its email and identifier from database.
    ///
    /// # Arguments
    ///
    /// * `email` - A string that holds the email of the hairdresser.
    pub fn get_hdresser_id(hdresser_email: &str) -> (String, String) {
        let index = "id";
        let query = format!(
            "SELECT hairdressers.id as {index} WHERE hairdressers.email = '{hdresser_email}'"
        );
        (query.to_owned(), index.to_string())
    }

    /// Returns a query for getting hairdressers that have works (photos) with given hairstyle.
    ///
    /// # Arguments
    ///
    /// * `hstyle` - A string that holds a hairstyle.
    pub fn get_hdressers_by_hstyle(
        hstyle: &str,
    ) -> (String, (String, String, String, String, String, String)) {
        let index = (
            "hd_id".to_string(),
            "hd_email".to_string(),
            "hd_name".to_string(),
            "hd_number".to_string(),
            "hd_address".to_string(),
            "hd_company".to_string(),
        );
        let query = format!(
            "
                SELECT DISTINCT hairdressers.id as {}, hairdressers.email as {}, hairdressers.name as {}, hairdressers.number as {}, hairdressers.address as {}, hairdressers.company as {}
                FROM style_to_dresser
                JOIN hairdressers ON hairdresser_id = hairdressers.id
                JOIN hairstyles ON hairstyle_name = hairstyles.name
                WHERE hairstyles.name = '{hstyle}'
            ",
            index.0, index.1, index.2, index.3, index.4, index.5
        );
        (query.to_owned(), index)
    }

    pub fn get_images_by_hdresser(hd_id: i64) -> String {
        let query = format!(
            "
                SELECT img_url as img FROM style_to_dresser
                WHERE hairdresser_id = '{hd_id}';
            "
        );
        query.to_owned()
    }

    /// Returns a query for getting hairdresser by its id.
    ///
    /// # Arguments
    ///
    /// * `hd_id` - A number that holds hairdresser's id in database.
    pub fn get_hdressers_by_id(hd_id: i64) -> String {
        let query = format!(
            "
                SELECT *
                FROM hairdressers
                WHERE id = '{hd_id}'
            "
        );
        query.to_owned()
    }

    /// Returns all picture urls from hairdresser by hairstyle.
    ///
    /// # Arguments
    ///
    /// * `hd_id` - A number that holds hairdresser's id in database.
    /// * `hstyle` - A string that holds a name of a hairstyle.
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

    /// Return a query that says if given email exists in databse;
    ///
    /// # Arguments
    ///
    /// * `email` - A string that holds the email to be checked.
    pub fn is_email_exist(email: &str) -> String {
        let query = format!(
            "
                SELECT id FROM hairdressers
                WHERE email = '{email}'
            "
        );
        query.to_owned()
    }

    /// Returns a query checking if password is correct.
    ///
    /// # Arguments
    ///
    /// * `hd_id` - A number that holds hairdresser's id in database.
    /// * `pass` - A string that holds the password of the hairdresser.
    pub fn get_password(hd_id: i64, pass: &str) -> String {
        let query = format!(
            "
                SELECT hairdressers.id as id, hairdressers.name as name
                FROM hairdressers
                JOIN dresser_login_info ON hairdressers.id = dresser_login_info.id
                WHERE hairdressers.id = {hd_id} AND password = '{pass}'
            "
        );
        query.to_owned()
    }

    /// Returns a query for saving photo in database;
    ///
    /// # Arguments
    ///
    /// * `hd_id` - A number that holds hairdresser's id in database.
    /// * `photo_name` - A string that holds photo's name.
    /// * `hstyle` - A string that holds a hairstyle for given photo.
    pub fn add_photo_to_db(hd_id: i64, photo_name: &str, hstyle: &str) -> String {
        let query = format!("INSERT INTO style_to_dresser (hairdresser_id, hairstyle_name, img_url) VALUES ('{hd_id}', '{hstyle}', '{hd_id}/{hstyle}/{photo_name}');");
        query.to_owned()
    }

    /// Returns a query for adding hairdresser to the database.
    ///
    /// # Arguments
    ///
    /// * `email` - A string that holds the email of the hairdresser.
    /// * `name` - A string that holds the name of the hairdresser.
    /// * `num` - A string that holds the phone number of the hairdresser.
    /// * `addr` - A string that holds the address of the hairdresser.
    /// * `com` - A string that holds the company of the hairdresser.
    pub fn add_user_to_db(email: &str, name: &str, num: &str, addr: &str, com: &str) -> String {
        let query = format!("INSERT INTO hairdressers (email, name, number, address, company) VALUES ('{email}', '{name}', '{num}', '{addr}', '{com}');");
        query.to_owned()
    }

    /// Returns a query for changing hairdresser's password.
    ///
    /// # Arguments
    ///
    /// * `hd_id` - A number that holds hairdresser's id in database.
    /// * `pass` - A string that holds the password of the hairdresser.
    pub fn change_password(hd_id: i64, pass: &str) -> String {
        let query = format!("INSERT INTO dresser_login_info VALUES ({hd_id}, '{pass}');");
        query.to_owned()
    }

    /// Returns a query for editing hairdresser's info.
    ///
    /// # Arguments
    ///
    /// * `hd_id` - A number that holds hairdresser's id in database.
    /// * `email` - A string that holds the email of the hairdresser.
    /// * `name` - A string that holds a name to the hairdresser.
    /// * `num` - A string slice that holds hairdresser's phone number.
    /// * `addr` - A string that holds the hairdresser's address.
    /// * `com` - A string that holds a name of a company where the hairdresser works.
    pub fn edit_hairdresser_info(
        hd_id: i64,
        email: &str,
        name: &str,
        num: &str,
        addr: &str,
        com: &str,
    ) -> String {
        let query = format!(
            "
                UPDATE hairdressers
                SET email = '{email}', name = '{name}', number = '{num}', address = '{addr}', company = '{com}'
                WHERE hairdressers.id = {};
            ", hd_id
        );
        query.to_owned()
    }
}
