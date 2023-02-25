use serde::Serialize;

#[derive(Serialize)]
pub struct Hairdresser {
    pub name: String,
    pub num: String,
    pub addr: String,
    pub company: String,
}

impl Hairdresser {
    pub fn new(name: String, phone_number: String, address: String, company: String) -> Self {
        Hairdresser {
            name: (name),
            num: (phone_number),
            addr: (address),
            company: (company),
        }
    }
}
