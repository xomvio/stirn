mod config;
mod utils;

use config::{get_config, Config};
use utils::{Server, stream_read};
use lazy_static::lazy_static;
lazy_static! {
    static ref CONFIG : Config = get_config();
}

fn main() {
    let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", CONFIG.port)).unwrap();

    while let Ok((stream, _)) = listener.accept() {
        std::thread::spawn(move || {
            let req = stream_read(&stream);
            let hostname = req.get_header("Host").unwrap();

            for server in CONFIG.servers.iter() {
                if server.url == hostname {
                    req.handle(stream, server.dir.clone());
                    break;
                }
            }
        });
    }
}

