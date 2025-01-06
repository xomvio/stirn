use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, BufReader};
pub mod request;
use request::Request;
pub mod response;

pub const RESPONSE_200:&str = "HTTP/1.1 200 OK\r\n";
pub const RESPONSE_404:&str = "HTTP/1.1 404 Not Found\r\n";
pub const RESPONSE_500:&str = "HTTP/1.1 500 Internal Server Error\r\n";

pub async fn stream_read_pure(stream:&mut TcpStream) -> [u8; 1024] {
    let mut reading_buffer = [0; 1024];
    //writing stream to buffer as bytes
    match BufReader::new(stream).read(&mut reading_buffer).await {
        Ok(_) => {},
        Err(e) => { log(format!("Critical: Cannot read stream. {}",e.to_string()).as_str());}
    }
    reading_buffer
}

pub async fn stream_read(stream:&mut TcpStream) -> Request {
    //reading buffer
    let mut reading_buffer = [0; 1024];
    //writing stream to buffer as bytes
    match BufReader::new(stream).read(&mut reading_buffer).await {
        Ok(_) => {},
        Err(e) => { log(format!("Critical: Cannot read stream. {}",e.to_string()).as_str());}
    }

    let buffer_str = String::from_utf8_lossy(&reading_buffer);	//convert buffer to string

    let buffer_lines: Vec<String> = buffer_str.split("\r\n").map(|s| s.to_string()).collect();	//headers
    let first_line: Vec<String> = buffer_lines[0].split_whitespace().map(|s| s.to_string()).collect();	//first line of header

    let method = match first_line[0].as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        _ => Method::OTHER
    };

    Request {
        method,
        endpoint: first_line[1].to_string(),
        https: false, 
        headers:  buffer_lines[1..].to_vec(),
        error: "".to_string(),
    }
}

pub fn log(message: &str) {
    println!("{}", message);
}

pub struct Server {
    pub name: String,
    pub hostname: String,
    pub port: u16,
    pub dir: String,
}

pub enum ServerType {
    Static(String),
    PortForwarded(u16),
    Stirner(u16),
}

pub enum Method {
    GET,
    POST,
    OTHER
}
impl Clone for Method {
    fn clone(&self) -> Self {
        match self {
            Method::GET => Method::GET,
            Method::POST => Method::POST,
            Method::OTHER => Method::OTHER,
        }
    }
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Server {
            name: self.name.clone(),
            hostname: self.hostname.clone(),
            port: self.port,
            dir: self.dir.clone(),
        }
    }
}