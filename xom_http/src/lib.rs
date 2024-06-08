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

	let mut reader = BufReader::new(&mut streamx);	//tcp streamreader

	let mut reading_buffer = [0; 1024];	//reading buffer
	let _ = reader.read(&mut reading_buffer).expect("cannot read stream for buffer");	//writing stream to buffer as bytes

	let buffer_str = String::from_utf8_lossy(&reading_buffer);	//convert buffer to string

	let buffer_lines:Vec<String> = buffer_str.split("\r\n").collect_vec().iter().map(|&s| s.into()).collect();	//headers
	let first_line:Vec<String> = buffer_lines[0].split_whitespace().collect_vec().iter().map(|&s| s.into()).collect();	//first line of header
	
	let req = Request {
		method: first_line[0].to_string(),
		endpoint: first_line[1].to_string(), 
		protocol:first_line[2].to_string(), 
		headers:  buffer_lines[1..].to_vec()
	};

	let mut resp:Response = Response { headers: String::new(), body: vec![] };
	resp.headers = format!("{}\r\npage not found: {}", RESPONSE_404.to_string(), req.endpoint);

	for route in routes {		//response if route exists on endpoint list

		if req.endpoint == route.endpoint {
			let typeistext;
			if req.endpoint.contains(".css") {
				resp.headers = RESPONSE_200.to_string() + "Content-Type: text/css\r\n";		//css file
				typeistext = true;
			}
			else if req.endpoint.contains(".ico"){
				resp.headers = RESPONSE_200.to_string() + "Content-Type: image/x-icon\r\n";	//ico file
				typeistext = false;
			}
			else {
				resp.headers = RESPONSE_200.to_string() + "Content-Type: text/html\r\n";	//html file
				typeistext = true;
			}


			if typeistext {				
				let filestr = initialize(route, &req);	//initialize file if endpoint matches

				let encoding_gzip;	//check gzip encoding
				match get_header(&req.headers, "Accept-Encoding") {
					Some(headerval)=> encoding_gzip = headerval.contains("gzip"),
					None=>encoding_gzip = false,
				}

				if encoding_gzip {
					resp.body = compress_data(filestr.as_bytes());
					resp.headers += "Content-Encoding: gzip\r\n";
				}
				else {	//no compression or unsupported
					resp.body = filestr.as_bytes().to_vec();
				}
			}
			else {
				if req.endpoint.contains(".ico") {
					let encoding_gzip;	//check gzip encoding
					match get_header(&req.headers, "Accept-Encoding") {
						Some(headerval)=> encoding_gzip = headerval.contains("gzip"),
						None=>encoding_gzip = false,
					}

					if encoding_gzip {
						let favicon = fs::read("files".to_string() + req.endpoint.as_str()).unwrap();
						resp.body = compress_data(&favicon);
						resp.headers += "Content-Encoding: gzip\r\n";
					}
					else {
						resp.body = fs::read("files".to_string() + req.endpoint.as_str()).unwrap();
					}					
				}
			}


			resp.headers += &("Content-Length: ".to_string() + &resp.body.len().to_string() + "\r\n\r\n");
			println!("{}",resp.headers);
			break;
		}
		else if req.endpoint == "/favicon.ico" {	//favicon is pre-defined. no routing needed.
			match fs::read("files/favicon.ico") {
				Ok(favicon)=>{
					resp.headers = RESPONSE_200.to_string() + "Content-Type: image/x-icon\r\nContent-Length: " + &favicon.len().to_string() + "\r\n\r\n";
					resp.body = favicon;
				}
				Err(_)=>{
					resp.headers = RESPONSE_404.to_string() + "\r\n";
				}
			}
			break;
		}
	}
	
    streamx.write_all(resp.headers.as_bytes()).unwrap();
    streamx.write_all(&resp.body).unwrap();
    streamx.flush().unwrap();
}
