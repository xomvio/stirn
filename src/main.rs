mod config;
mod utils;

use std::io::stdin;
use config::{get_config, Config};
use utils::{Server, stream_read};

static mut SERVERS : Vec<Server> = Vec::new();

fn main() { //this is actually lib.rs start function
    let config = get_config();
    let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).unwrap();
    unsafe { SERVERS = config.servers.clone(); }
    //let _ = start_servers(config.clone());
    //let servers = config.servers.clone();

    while let Ok((stream, _)) = listener.accept() {
        std::thread::spawn(move || {
            let req = stream_read(&stream);
            let hostname = req.get_header("Host").unwrap();

            for server in unsafe { &SERVERS } {
                if server.url == hostname {
                    req.handle(stream, server.dir.clone());
                    break;
                }
            }
        });
    }
    
}

async fn start_servers(config: Config) {
    for server in config.servers {
        std::thread::spawn(move || server.run());
    }
    //it just means wait for ctrl-c or etc.
    for _ in stdin().lines() { } 
}