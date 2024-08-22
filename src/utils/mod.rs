use std::net::TcpStream;
use std::io::{BufReader, Read, Write};
use itertools::Itertools;
pub mod request;

pub const RESPONSE_200:&str = "HTTP/1.1 200 OK\r\n";
pub const RESPONSE_404:&str = "HTTP/1.1 404 Not Found\r\n";

pub fn stream_read(mut stream:&TcpStream) -> Request {        
    let mut reader = BufReader::new(&mut stream);   //tcp reader
    let mut reading_buffer = [0; 1024];	//reading buffer
    let _ = reader.read(&mut reading_buffer).expect("cannot read stream for buffer");	//writing stream to buffer as bytes

    let buffer_str = String::from_utf8_lossy(&reading_buffer);	//convert buffer to string

    let buffer_lines:Vec<String> = buffer_str.split("\r\n").collect_vec().iter().map(|&s| s.into()).collect();	//headers
    let first_line:Vec<String> = buffer_lines[0].split_whitespace().collect_vec().iter().map(|&s| s.into()).collect();	//first line of header

    Request {
        method: first_line[0].to_string(),
        endpoint: first_line[1].to_string(),
        protocol:first_line[2].to_string(), 
        headers:  buffer_lines[1..].to_vec(),
        //nekot: "".to_string(),
    }
}

#[derive(Debug)]
pub struct Request {
	pub method: String,
	pub endpoint: String,
	pub protocol: String,
	pub headers: Vec<String>,
    //pub nekot: String,
}

pub struct Response {
    pub headers: String,
    pub body: Vec<u8>,
    pub stream: TcpStream,
}

impl Response {
    pub fn send(mut self) -> TcpStream {
        self.stream.write_all(self.headers.as_bytes()).unwrap();
        self.stream.write_all(&self.body).unwrap();
        self.stream.flush().unwrap();
        self.stream
    }
}

#[derive(Clone)]
pub struct Server {
    pub name: String,
    pub url: String,
    pub port: u16,
    pub dir: String,
}