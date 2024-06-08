use std::net::TcpListener;
pub mod utils;
use utils::{stream, HttpBuilder};

pub fn run(httpbuilder:HttpBuilder) {
	let listener = TcpListener::bind(
		"127.0.0.1:".to_string()	//localhost
		+ httpbuilder.port.to_string().as_str()).unwrap();	//port from httpbuilder
    
	for isstream in listener.incoming() {
		match isstream {
			Ok(streamx) => {
				let routes = httpbuilder.routes.clone();
				std::thread::spawn(move || stream::handle(streamx, routes));
			}
			Err(e) => {
				println!("error: {}", e);
			}
		}
	}
}
