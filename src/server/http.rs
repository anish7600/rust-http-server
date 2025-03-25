use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_message: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(
        status_code: u16, 
        status_message: String, 
        headers: Option<HashMap<String, String>>, 
        body: Vec<u8>
    ) -> Self {
        HttpResponse {
            status_code,
            status_message,
            headers: headers.unwrap_or_default(),
            body,
        }
    }

    pub fn ok_html(body: &str) -> Self {
        Self::new(
            200, 
            "OK".to_string(), 
            Some([("Content-Type".to_string(), "text/html".to_string())].iter().cloned().collect()),
            body.as_bytes().to_vec()
        )
    }

    pub fn ok_json(body: &str) -> Self {
        Self::new(
            200, 
            "OK".to_string(), 
            Some([("Content-Type".to_string(), "application/json".to_string())].iter().cloned().collect()),
            body.as_bytes().to_vec()
        )
    }

    pub fn not_found() -> Self {
        Self::new(
            404, 
            "Not Found".to_string(), 
            None,
            b"404 - Route Not Found".to_vec()
        )
    }

    pub fn bad_request() -> Self {
        Self::new(
            400, 
            "Bad Request".to_string(), 
            None,
            b"400 - Bad Request".to_vec()
        )
    }
}
