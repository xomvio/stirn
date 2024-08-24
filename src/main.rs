mod config;
mod utils;

use config::{get_config, Config};
use utils::{response::ResponseBuilder, stream_read, Server, RESPONSE_500};
use lazy_static::lazy_static;
lazy_static! {
    static ref CONFIG : Config = get_config();
}

fn main() {
    let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", CONFIG.port)).unwrap();

    while let Ok((stream, _)) = listener.accept() {
        std::thread::spawn(move || {
            let mut req = stream_read(&stream);
            let hostname = match req.get_header("Host") {
                Some(host) => host,
                None => { req.error = "Host header not found in request".to_string();
                    ResponseBuilder { dir: "".to_string(), endpoint: "".to_string(), is_gzip: false, content_type: "text/html".to_string(), stream: stream, status: RESPONSE_500.to_string(), error: req.error }.build().send();
                    return;
                }
            };

            for server in CONFIG.servers.iter() {
                if server.url == hostname {
                    req.handle(stream, server.dir.clone());
                    break;
                }
            }
        });
    }
}

