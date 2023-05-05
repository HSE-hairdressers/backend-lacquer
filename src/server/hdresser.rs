use serde::{Deserialize, Serialize};

/// A stylish human beign part represented here.
#[derive(Serialize, Debug, Default, Clone)]
pub struct Hairdresser {
    /// Every hairdresser has unique `id` in order to represente them in database.
    id: i64,
    /// Every hairdresser should have an email that will help other people communicate with them.
    email: String,
    /// Pic field is optional. Hairdresser can set up picture to show it in the profile.
    pub pic: String,
    /// Every human should have a name.
    pub name: String,
    /// Num field is optional. Not every hairdresser has social skills to communicate.
    pub num: String,
    /// If hairdresser wants to have clients then should share his address.
    pub addr: String,
    /// Company field is optional. Hairdresser may working by his own at home.
    pub company: String,
}

impl Hairdresser {
    /// Returns a hairdresser with the given data.
    ///
    /// # Arguments
    ///
    /// * `id` - A number that holds hairdresser's id in database.
    /// * `email` - A string that holds the email of the hairdresser.
    /// * `pic` - A string that holds a URL to the hairdresser's picture.
    /// * `num` - A string slice that holds hairdresser's phone number.
    /// * `addr` - A string that holds the hairdresser's address.
    /// * `company` - A string that holds a name of a company where the hairdresser works.
    pub fn new(
        id: i64,
        email: String,
        name: String,
        phone_number: String,
        address: String,
        company: String,
    ) -> Self {
        Hairdresser {
            id: (id),
            email: (email),
            pic: "http://clipart-library.com/images/BTaroLj5c.png".to_string(),
            name: (name),
            num: (phone_number),
            addr: (address),
            company: (company),
        }
    }

    
    /// Returns a hairdresser with the given id.
    ///
    /// # Arguments
    ///
    /// * `id` - A number that holds hairdresser's id in database.
    pub fn with_id(id: i64) -> Self {
        Self {
            id: (id),
            ..Self::default()
        }
    }

    /// Setter for the email.
    pub fn set_email(&mut self, email: &str) {
        self.email = email.to_string();
    }
    /// Setter for the name.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    /// Setter for the phone number;
    pub fn set_num(&mut self, num: &str) {
        self.num = num.to_string();
    }
    /// Setter for the address.
    pub fn set_address(&mut self, addr: &str) {
        self.addr = addr.to_string();
    }
    /// Setter for the company.
    pub fn set_company(&mut self, company: &str) {
        self.company = company.to_string();
    }

    // Getter for the hairdresser's id.
    pub fn get_id(&self) -> i64 {
        self.id
    }
    // Getter for the hairdresser's email.
    pub fn get_email(&self) -> &str {
        &self.email
    }
    // Getter for the hairdresser's name.
    pub fn get_name(&self) -> &str {
        &self.name
    }
    // Getter for the hairdresser's phone number.
    pub fn get_number(&self) -> &str {
        &self.num
    }
    // Getter for the hairdresser's address.
    pub fn get_address(&self) -> &str {
        &self.addr
    }
    // Getter for the hairdresser's company.
    pub fn get_company(&self) -> &str {
        &self.company
    }
}

/// There are a lot of hairdressers in database. But how we can recognize who is who?
/// The good news is that we only need 2 fields -> `id` abd `name`
#[derive(Serialize, Deserialize, Debug)]
pub struct HairdresserIdentity {
    id: i64,
    pub name: String,
}

impl HairdresserIdentity {
    /// Returns a hairdresser identity with the given data.
    ///
    /// # Arguments
    ///
    /// * `id` - A number that holds hairdresser's id in database.
    /// * `num` - A string slice that holds hairdresser's phone number.
    pub fn new(id: i64, name: String) -> Self {
        Self {
            id: (id),
            name: (name),
        }
    }
}
