//use std::net::TcpStream;
use tokio::net::TcpStream;
use crate::CONFIG;

use super::{log, response::ResponseBuilder};

pub struct Request {
	pub method: String,
	pub endpoint: String,
	pub https: bool,
	pub headers: Vec<String>,
    pub error: String,
}

pub enum Method {
    GET,
    POST,
    OTHER
}

impl Request {
    pub fn get_header(&mut self, header: &str) -> Option<&str> {
        for line in self.headers.iter() {
            let mut name_and_val = line.split(": ");
            if name_and_val.next().expect("broken header key") == header {
                //return Some(name_and_val.next().expect("broken header value"));
                return match name_and_val.next() {
                    Some(val) => Some(val),
                    None => {self.error = format!("broken header value: {}", line); None},
                }
            }
        }
        log(&format!("Warning: Header {} not found in request headers; {}", header, self.headers.join("  ")));
        None
    }

    pub fn accepts_gzip(&mut self) -> bool {
        return self.get_header("Accept-Encoding").is_some_and(|headerval| headerval.contains("gzip"));
    }

    // Handle a connection on the specified TCP stream.
    pub async fn handle(&mut self, stream: TcpStream, dir: String) {
        let endpoint = if self.endpoint == "/" { "/index.html" } else { &self.endpoint }.to_string();

        let content_type = CONFIG.mimetypes
            .get(endpoint.split('.').last().unwrap_or(""))
            .unwrap_or(&CONFIG.default_mimetype).to_string();

        let is_gzip = self.accepts_gzip();
        let status = if self.error.len() == 0 { super::RESPONSE_200 } else { super::RESPONSE_500 }.to_string();
        let error = self.error.clone();

        ResponseBuilder {
            stream,
            content_type,
            dir,
            endpoint,
            is_gzip,
            status,
            error,
        }.build().send().await;
    }
}
