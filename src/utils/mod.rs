//use std::net::TcpStream;
//use std::io::{BufReader, Read};
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};
//use itertools::Itertools;
pub mod request;
use request::Request;
pub mod response;

pub const RESPONSE_200:&str = "HTTP/1.1 200 OK\r\n";
pub const RESPONSE_206:&str = "HTTP/1.1 206 Partial Content\r\n";
pub const RESPONSE_404:&str = "HTTP/1.1 404 Not Found\r\n";
pub const RESPONSE_500:&str = "HTTP/1.1 500 Internal Server Error\r\n";

pub async fn stream_read(stream:&mut TcpStream) -> Request {
    let mut reading_buffer = [0; 1024];	//reading buffer
    //writing stream to buffer as bytes
    match BufReader::new(stream).read(&mut reading_buffer).await {
        Ok(_) => {},
        Err(e) => { log(format!("Critical: Cannot read stream. {}",e.to_string()).as_str());}
    }

    let buffer_str = String::from_utf8_lossy(&reading_buffer);	//convert buffer to string

    let buffer_lines: Vec<String> = buffer_str.split("\r\n").map(|s| s.to_string()).collect();	//headers
    let first_line: Vec<String> = buffer_lines[0].split_whitespace().map(|s| s.to_string()).collect();	//first line of header

    Request {
        method: first_line[0].to_string(),
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
