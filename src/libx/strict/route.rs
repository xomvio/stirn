use flate2::{write::GzEncoder, Compression};
use std::io::{Write, Read};
use std::fs::File;

use super::{Route, RESPONSE_404, RESPONSE_200};
use crate::libx::{Request, Response};


impl Route {
    pub fn respond(self, req: &Request) -> Response {
        let mut resp:Response = Response { headers: String::new(), body: vec![] };
        resp.headers = format!("{}\r\npage not found: {}", RESPONSE_404.to_string(), req.endpoint);
    		
        let file_extension = self.file.split('.').last().unwrap_or("function");
        let content_type = match file_extension {
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
        resp.headers = format!("{RESPONSE_200}Content-Type: {content_type}\r\n");

        let filestr = self.get_page();

        if req.is_gzip_accepted() {
            resp.body = compress_data(filestr.as_bytes());
            resp.headers += "Content-Encoding: gzip\r\n";
        }
        else {
            resp.body = filestr.as_bytes().to_vec();
        }

        resp.headers += &("Content-Length: ".to_string() + &resp.body.len().to_string() + "\r\n\r\n");
        println!("{}",resp.headers);
        resp
    }


    fn get_page(&self) -> String {
        read_from_file(&self.endpoint)
    }
}

pub fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

fn read_from_file(path: &str) -> String {
    //if path == "" { return "".to_string(); }

    let mut file = File::open("files/".to_string() + path).unwrap();
    let mut filestr: String = String::new();
    file.read_to_string(&mut filestr).unwrap();
    filestr
}