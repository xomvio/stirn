use std::fs;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use itertools::Itertools;
use super::*;
use common::compress_data;

pub fn handle(mut streamx: TcpStream, routes:Vec<Route>) {

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

	let mut found = false;

	//let mut route:Route = Route { endpoint: "", layout: (), file: () }

	for route in routes {		//response if route exists on endpoint list

		if req.endpoint == route.endpoint {
			let resp = route.clone().init(&req, route.commands);
			streamx = stream_write(streamx, resp);

			found = true;
			break;
		}

	}

	if !found {
		let mut resp:Response = Response { headers: String::new(), body: vec![] };
		resp.headers = format!("{}\r\npage not found: {}", RESPONSE_404.to_string(), req.endpoint);
		stream_write(streamx, resp);
	}
}
