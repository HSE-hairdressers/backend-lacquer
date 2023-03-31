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
    pub fn init_db() -> String {
        let query = "
            CREATE TABLE hairdressers (
                                        id UNIQUE AUTOINCREMENT INTEGER PRIMARY,
                                        email TEXT UNIQUE,
                                        name TEXT NOT NULL
                                        number TEXT,
                                        address TEXT,
                                        company TEXT,
            );
            CREATE TABLE hairstyles (name TEXT UNIQUE PRIMARY);
            CREATE TABLE style_to_dresser (
                                        hairdresser_id INTEGER,
                                        hairstyle_name TEXT,
                                        img_url TEXT,
                                        FOREIGN KEY (hairdresser_id) REFERENCES
                                        hairdressers(id),
                                        FOREIGN KEY (hairstyle_name) REFERENCES hairstyles(name)
            );
            INSERT INTO hairdressers (email, name) VALUES ('khadiev.edem@gmail.com', 'Khadiev Edem');
            INSERT INTO hairdressers (email, name) VALUES ('max.md8@gmail.com', 'Maxim Dudarev');
            INSERT INTO hairdressers (email, name) VALUES ('ageev.maxim2003@gmail.com', 'Maxim Ageev');

        ";
        query.to_owned()
    }

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
                SELECT hairdressers.id as {}, hairdressers.email as {}, hairdressers.name as {}
                FROM style_to_dresser
                JOIN hairdressers ON hairdresser_email = hairdressers.email
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
                JOIN hairstyles ON hairstyle_id = hairstyles.id
                JOIN hairdressers ON hairdresser_email = hairdressers.email
                WHERE hairdresser_id = '{hd_id}' AND hairstyle_name ='{hstyle}'
            "
        );
        (query.to_owned(), index.to_string())
    }
}
