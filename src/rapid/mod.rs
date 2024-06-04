use std::io::Write;
use std::net::TcpStream;

pub const RESPONSE_404:&str = "HTTP/1.1 404 Not Found\r\n";
pub const RESPONSE_200:&str = "HTTP/1.1 200 OK\r\n";
pub const RESPONSE_201:&str = "HTTP/1.1 201 Created\r\n";
pub struct Request {
	pub method: String,
	pub endpoint: String,
	pub protocol: String,
	pub headers: Vec<String>
}

pub struct Response {
    pub headers: String,
    pub body: Vec<u8>,
}

pub fn get_header(lines:&Vec<String>, key:&str) -> String {
    for l in lines {
        let mut name_and_val = l.split(": ");
        if name_and_val.next().expect("broken header key") == key {
            return name_and_val.next().expect("broken header value").to_string();
        }
    }
    return "undefined".to_string()
}
