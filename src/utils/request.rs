use super::Request;
use super::Response;
use std::net::TcpStream;
use flate2;
use std::io::Write;
use std::fs;

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
        //let req = Request::read(&stream);
        println!("{}", self.endpoint);
        let endpoint = if self.endpoint == "/" { "/index.html" } else { &self.endpoint };

        //let file_extension = endpoint.split('.').last().unwrap_or("txt");
        let content_type = match endpoint.split('.').last().unwrap() {
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
            _ => "text/plain",
        };

        let filestr = fs::read(format!("{}/{}", dir, endpoint));
        match filestr {
            Ok(filestr) => {
                if self.is_gzip_accepted() {
                    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
                    encoder.write_all(&filestr).unwrap();
                    let body = encoder.finish().unwrap();
                    Response { 
                        headers: format!("{}Content-Type: {}\r\nContent-Encoding: gzip\r\nContent-Length: {}\r\n\r\n", super::RESPONSE_200, content_type, body.len()), 
                        body,
                        stream
                    }
                }
                else {
                    let body = filestr;
                    Response { 
                        headers: format!("{}Content-Type{}\r\nContent-Length: {}\r\n\r\n", super::RESPONSE_200, content_type, body.len()), 
                        body,
                        stream
                    }
                }
            }
            Err(_) => {
                let mut resp: Response = Response { headers: String::new(), body: vec![] , stream};
                resp.headers = format!("{}\r\npage not found: {}", super::RESPONSE_404, endpoint);
                resp
            }
        }
        .send();
        //resp.send();
        //super::stream_write(stream, resp);
    }

}
