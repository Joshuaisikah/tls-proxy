use serde::Serialize;

#[derive(Serialize)]
pub struct Report {
    pub timestamp: String,
    pub host: String,
    pub status: u16,
    pub path: String,
    pub method: String,
    pub target: String,
    pub response_time: String,
    pub tls_version: String,
    pub cipher_suite: String,
    pub cert_valid: bool,
}
impl Report {
    pub fn new() -> Self {
        Self {
            timestamp: String::new(),
            host: String::new(),
            status: 0,
            path: String::new(),
            method: String::new(),
            target: String::new(),
            response_time: String::new(),
            tls_version: String::new(),
            cipher_suite: String::new(),
            cert_valid: false,
        }
    }
    pub fn write_log(&self) {
     let log =serde_json::to_string(&self).unwrap();
        println!("{}", log);
    }

}