use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default, Clone)]
pub struct Hairdresser {
    id: i64,
    email: String,
    pub pic: String,
    pub name: String,
    pub num: String,
    pub addr: String,
    pub company: String,
}

impl Hairdresser {
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

    pub fn with_id(id: i64) -> Self {
        Self {
            id: (id),
            ..Self::default()
        }
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = email.to_string();
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn set_num(&mut self, num: &str) {
        self.num = num.to_string();
    }
    pub fn set_address(&mut self, addr: &str) {
        self.addr = addr.to_string();
    }
    pub fn set_company(&mut self, company: &str) {
        self.company = company.to_string();
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_number(&self) -> &str {
        &self.num
    }
    pub fn get_address(&self) -> &str {
        &self.addr
    }
    pub fn get_company(&self) -> &str {
        &self.company
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HairdresserIdentity {
    id: i64,
    pub name: String,
}

impl HairdresserIdentity {
    pub fn new(id: i64, name: String) -> Self {
        Self {
            id: (id),
            name: (name),
        }
    }
}
