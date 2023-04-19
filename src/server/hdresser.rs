use serde::Serialize;

#[derive(Serialize, Debug)]
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

    pub fn get_id(&self) -> i64 {
        self.id
    }
}

#[derive(Serialize, Debug)]
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
