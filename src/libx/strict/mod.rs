use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read, Write};
use itertools::Itertools;
use super::{RESPONSE_200, RESPONSE_404};

use super::Request;
use super::Response;
mod route;

static mut ROUTES:Vec<Route> = vec![];

#[derive(Clone)]
pub struct Route {
    pub endpoint: String,
    pub is_file: bool,
    pub file: String,
}

pub struct StrictServer {
    pub port: u16,
    pub routes: Vec<Route>,
    pub error_page: Route,
}

impl StrictServer {
    pub fn run(&self) {
        unsafe { ROUTES = self.routes.clone(); }
        
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    std::thread::spawn(|| handle_strict(stream));
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

/// Handle a connection on the specified TCP stream.
fn handle_strict(mut stream: std::net::TcpStream) {
    let req = crate::libx::Request::read(&mut stream);

	let mut found = false;

    unsafe {
        for route in ROUTES.iter() { //response if route exists on endpoint list
            if req.endpoint == route.endpoint {
                let resp = route.clone().respond(&req);
                stream = stream_write(stream, resp);
                found = true;
                break;
            }
        }
    }

	if !found {
		let mut resp: Response = Response { headers: String::new(), body: vec![] };
		resp.headers = format!("{}\r\npage not found: {}", RESPONSE_404.to_string(), req.endpoint);
		stream_write(stream, resp);
	}
}


pub fn stream_write(mut stream:TcpStream, resp:Response) -> TcpStream {
    stream.write_all(resp.headers.as_bytes()).unwrap();
    stream.write_all(&resp.body).unwrap();
    stream.flush().unwrap();
    stream
}

