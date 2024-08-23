use std::io::Write;
use std::net::TcpStream;
use std::fs;

use super::RESPONSE_404;

pub struct Response {
    pub headers: String,
    pub body: Vec<u8>,
    pub stream: TcpStream,
}

impl Response {
    pub fn send(mut self) {
        self.stream.write_all(self.headers.as_bytes()).unwrap();
        self.stream.write_all(&self.body).unwrap();
        self.stream.flush().unwrap();
    }
}

pub struct ResponseBuilder {
    pub dir : String,
    pub endpoint : String,
    pub is_gzip : bool,
    pub content_type : String,
    pub stream : TcpStream,
    pub status : String,
}

impl ResponseBuilder {    
    pub fn build(mut self) -> Response {
        if self.endpoint == "/500pls" {
            return ResponseBuilder::error_500(self)
        }
        let filestr = fs::read(format!("{}/{}", self.dir, self.endpoint));
        match filestr {
            Ok(filestr) => {
                let (gzipstr, body) = 
                if self.is_gzip {
                    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
                    encoder.write_all(&filestr).unwrap();
                    let body = encoder.finish().unwrap();
                    ("Content-Encoding: gzip\r\n", body) 
                } 
                else {
                    ("", filestr) 
                };

                Response {
                    headers: format!("{}Content-Type: {}\r\nContent-Length: {}\r\n{}\r\n", self.status, self.content_type, body.len(), gzipstr), 
                    body,
                    stream: self.stream
                }
            },
            Err(_) => {
                ResponseBuilder::build(ResponseBuilder { dir: self.dir, endpoint: "404.html".to_string(), is_gzip: self.is_gzip, content_type: self.content_type, stream: self.stream, status: RESPONSE_404.to_string() }) 
            }
        }
    }

    pub fn error_500(self) -> Response {        
        //ResponseBuilder::build(Self { dir: self.dir, endpoint: "500.html".to_string(), is_gzip: false, content_type: "text/html".to_string(), stream: self.stream })
        Response { 
            headers: format!("{}Content-Type: text/html\r\nContent-Length: 25\r\n\r\n", super::RESPONSE_500), 
            body: "500 internal server error".as_bytes().to_vec(), 
            stream: self.stream 
        }
    }
}