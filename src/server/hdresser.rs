use serde::Serialize;

#[derive(Serialize)]
pub struct Hairdresser {
    id: i64,
    email: String,
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
