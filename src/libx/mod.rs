use std::{fs, net::{TcpListener, TcpStream}};
use itertools::Itertools;
use std::io::{BufReader, Read, Write};

pub mod strict;
pub mod request;

pub const RESPONSE_200:&str = "HTTP/1.1 200 OK\r\n";
pub const RESPONSE_404:&str = "HTTP/1.1 404 Not Found\r\n";

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
}


pub struct Server {
    pub name: String,
    pub url: String,
    pub port: u16,
    pub dir: String,
}

static mut DIR: String = String::new();

impl Server {
    pub fn run(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        println!("Listening on http://{}", listener.local_addr().unwrap());

        for (stream, addr) in listener.accept() {
            println!("Accepted connection from {}", addr);
            let dir = self.dir.clone();
            std::thread::spawn(|| handle(dir, stream));
            /*match stream {                
                Ok(stream) => {
                    let dir = self.dir.clone(); //must have a better solution
                    std::thread::spawn(|| handle(dir, stream));
                }
                Err(e) => println!("Error: {}", e),
            }*/
        }
    }

}

// Handle a connection on the specified TCP stream.
fn handle(dir: String, mut stream: std::net::TcpStream) {
    let req = request::stream_read(&mut stream);
    let endpoint = if req.endpoint == "/" { "/index.html" } else { &req.endpoint };

    let filestr = fs::read_to_string(format!("{}/{}", dir, endpoint));
    println!("{}{}", dir, endpoint);
    let resp = match filestr {
        Ok(filestr) => {            
            let mut resp: Response = Response { headers: String::new(), body: filestr.as_bytes().to_vec() };
    		resp.headers = format!("{}Content-Length: {}\r\n\r\n", RESPONSE_200, resp.body.len());
            resp
        }
        Err(_) => {
            let mut resp: Response = Response { headers: String::new(), body: vec![] };
    		resp.headers = format!("{}\r\npage not found: {}", RESPONSE_404.to_string(), endpoint);
            resp
        }
    };

    
    stream_write(stream, resp);
}


pub fn stream_write(mut stream:TcpStream, resp:Response) -> TcpStream {
    stream.write_all(resp.headers.as_bytes()).unwrap();
    stream.write_all(&resp.body).unwrap();
    stream.flush().unwrap();
    stream
}

//will use this later
pub fn get_header(lines:&Vec<String>, header:&str) -> Option<String> { //example key "Accept-Encoding"
    for l in lines {
        let mut name_and_val = l.split(": ");
        if name_and_val.next().expect("broken header key") == header {
            return Some(name_and_val.next().expect("broken header value").to_string());
        }
    }
    return None
}