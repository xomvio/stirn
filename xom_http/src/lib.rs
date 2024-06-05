use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use itertools::Itertools;

pub mod rapid;
use rapid::*;

//pls


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
	}

	/*if req.method == "GET" {
		if req.endpoint == "/" {
			resp.headers = RESPONSE_200.to_string() + "\r\n";
		}
		else if req.endpoint.starts_with("/echo/") {
			let echostr = first_line[1].replace("/echo/", "");
			if get_header(&req.headers, "Accept-Encoding").contains("gzip") {
				let echo = echostr.as_bytes();
                let mut compbody = Vec::new();
                {
                    let mut encoder = GzEncoder::new(&mut compbody, Compression::default());
                    encoder.write_all(echo).unwrap();
                    encoder.finish().unwrap();
                }
                resp.body = compbody;
                resp.headers = format!(
                    "{}Content-Encoding: gzip\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n",
                    RESPONSE_200,
                    resp.body.len()
                );

			}
			else {
				resp.body = echostr.as_bytes().to_vec();
				resp.headers = RESPONSE_200.to_string() + "Content-Type: text/plain\r\nContent-Length: " + &resp.body.len().to_string() + "\r\n\r\n";
			}
		}
		else if req.endpoint.starts_with("/files/") {
			let path = "/tmp/data/codecrafters.io/http-server-tester/".to_string() + req.endpoint.replace("/files/", "").as_str();

			let f = fs::read_to_string(path);
			match f {
				Ok(f) => {
					resp.headers = RESPONSE_200.to_string() + "Content-Type: application/octet-stream\r\nContent-Length: " + f.len().to_string().as_str() + "\r\n\r\n";
					resp.body = f.as_bytes().to_vec();
				}
				Err(e) => {
					println!("{}", e);
					resp.headers = RESPONSE_404.to_string() + "\r\n";

				}

			}
		}
		else if req.endpoint == "/user-agent" {
			resp.body = get_header(&req.headers,"User-Agent").as_bytes().to_vec();
			resp.headers = RESPONSE_200.to_string() + "Content-Type: text/plain\r\nContent-Length: " + &resp.body.len().to_string() + "\r\n\r\n";
		}
		else {
			resp.headers = RESPONSE_404.to_string() + "\r\n";
		}
	}
	else if req.method == "POST" {
		if req.endpoint.starts_with("/files/") {
			let path = "/tmp/data/codecrafters.io/http-server-tester/".to_string() + req.endpoint.replace("/files/", "").as_str();			
			let content: String = req.headers.last().unwrap().chars().filter(|&c| c != '\x00').collect();
			//println!("{content}");
			let mut f = File::create(path).unwrap();
			
			f.write(content.as_bytes()).unwrap();

			resp.headers = RESPONSE_201.to_string() + "\r\n";
		}
		else {
			resp.headers = RESPONSE_404.to_string() + "\r\n";
		}
	}
	else {
		resp.headers = RESPONSE_404.to_string() + "\r\n";
	}*/
	
    streamx.write_all(resp.headers.as_bytes()).unwrap();
    streamx.write_all(&resp.body).unwrap();

    if let Err(e) = streamx.flush() {
        println!("Error flushing stream: {}", e);
    }
}
