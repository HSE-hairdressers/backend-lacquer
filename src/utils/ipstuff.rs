use std::env;

pub struct IpAndPort {
    pub ip: String,
    pub port: u16,
}

impl IpAndPort {
    pub fn new() -> Self {
        let mut res = IpAndPort::default();
        if let Ok(a) = env::var("IP_ADDRESS") {
            res.ip = a;
        }
        if let Ok(a) = env::var("PORT") {
            if let Ok(b) = a.parse() {
                res.port = b;
            }
        }
        res
    } 

    fn default() -> Self {
        IpAndPort{ 
            ip: "localhost".to_string(), 
            port: 8011,
        }
    }
}

