use std::fs;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use itertools::Itertools;

pub mod rapid;
use rapid::*;

pub fn run(httpbuilder:HttpBuilder) {
	let listener = TcpListener::bind(
		"127.0.0.1:".to_string()	//localhost
		+ httpbuilder.port.to_string().as_str()).unwrap();	//port from httpbuilder
    
	for isstream in listener.incoming() {
		match isstream {
			Ok(streamx) => {
				println!("accepted new connection");
				let routes = httpbuilder.routes.clone();
				std::thread::spawn(move || handle_req(streamx, routes));
			}
			Err(e) => {
				println!("error: {}", e);
			}
		}
	}
}

fn handle_req(mut streamx: TcpStream, routes:Vec<Route>) {

	let mut reader = BufReader::new(&mut streamx);

	let mut reading_buffer = [0; 1024];
	let _ = reader.read(&mut reading_buffer).expect("cannot read stream for buffer");

	let buffer_str = String::from_utf8_lossy(&reading_buffer);

	let buffer_lines:Vec<String> = buffer_str.split("\r\n").collect_vec().iter().map(|&s| s.into()).collect();
	let first_line:Vec<String> = buffer_lines[0].split_whitespace().collect_vec().iter().map(|&s| s.into()).collect();
	
	let req = Request {
		method: first_line[0].to_string(),
		endpoint: first_line[1].to_string(), 
		protocol:first_line[2].to_string(), 
		headers:  buffer_lines[1..].to_vec()
	};

	let mut resp:Response = Response { headers: String::new(), body: vec![] };

	resp.headers = RESPONSE_404.to_string() + "\r\n";
	for route in routes {
		if req.endpoint == route.endpoint {
			resp.headers = RESPONSE_200.to_string() + "\r\n";
			let filestr = initialize(route);
			resp.body = filestr.as_bytes().to_vec();
		}
		else if req.endpoint == "/favicon.ico" {
			match fs::read("files/favicon.ico") {
				Ok(favicon)=>{
					resp.headers = RESPONSE_200.to_string() + "Content-Type: image/x-icon\r\nContent-Length: " + &favicon.len().to_string() + "\r\n\r\n";
					resp.body = favicon;
				}
				Err(_)=>{
					resp.headers = RESPONSE_404.to_string() + "\r\n";
				}
			}
		}
	}
	
    streamx.write_all(resp.headers.as_bytes()).unwrap();
    streamx.write_all(&resp.body).unwrap();
    streamx.flush().unwrap();
}
