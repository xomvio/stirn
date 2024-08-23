use std::net::TcpStream;
use flate2;
use std::io::Write;
use std::fs;

use super::response::{Response, ResponseBuilder,};

pub struct Request {
	pub method: String,
	pub endpoint: String,
	pub protocol: String,
	pub headers: Vec<String>,
}

impl Request {
    pub fn get_header(&self, header: &str) -> Option<&str> {
        for line in self.headers.iter() {
            let mut name_and_val = line.split(": ");
            if name_and_val.next().expect("broken header key") == header {
                return Some(name_and_val.next().expect("broken header value"));
            }
        }
        None
    }

    pub fn is_gzip_accepted(&self) -> bool {
        return self.get_header("Accept-Encoding").is_some_and(|headerval| headerval.contains("gzip"));
    }

    // Handle a connection on the specified TCP stream.
    pub fn handle(&self, stream: TcpStream, dir: String) {
        let endpoint = if self.endpoint == "/" { "/index.html" } else { &self.endpoint }.to_string();

        let content_type = match endpoint.split('.').last().unwrap_or("") {
            "css" => "text/css",
            "ico" => "image/x-icon",
            "html" => "text/html",
            "txt" | "text" => "text/plain",
            "js" => "text/javascript",
            "json" => "application/json",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "woff" => "font/woff",
            "function" => "text/html",
            _ => "text/html",
        }.to_string();

        ResponseBuilder {
            stream,
            content_type,
            dir,
            endpoint,
            is_gzip: self.is_gzip_accepted(),
            status: super::RESPONSE_200.to_string(),
        }.build().send();
    }
}
