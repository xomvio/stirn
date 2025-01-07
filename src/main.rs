mod config;
mod utils;
use std::io::Read;

use config::{get_config, Config};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use utils::{response::{Response, ResponseBuilder}, stream_read, stream_read_raw, Server, RESPONSE_500};
use lazy_static::lazy_static;
lazy_static! {
    static ref CONFIG: Config = get_config();
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", CONFIG.port)).await.unwrap();

    while let Ok((mut stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let mut req = stream_read(&mut stream).await;
            let hostname = match req.get_header("Host") {
                Some(host) => host,
                None => {
                    ResponseBuilder {
                        dir: "".to_string(),
                        endpoint: "".to_string(),
                        is_gzip: false,
                        mimetype: "text/html".to_string(),
                        stream,
                        method: req.method,
                        status: RESPONSE_500.to_string(),
                        error: "Host header not found in request".to_string(),
                    }
                    .build()
                    .send().await;
                    return;
                }
            };
            for server in CONFIG.servers.iter() {                
                if server.hostname == hostname {
                    if server.port != 0 {
                        let mut forwardstream = tokio::net::TcpStream::connect(format!("localhost:{}", server.port)).await.unwrap();
                        let _ = forwardstream.write_all(req.raw.as_bytes()).await.unwrap();
                        let mut reader = tokio::io::BufReader::new(&mut forwardstream);
                        let mut buf: Vec<u8> = Vec::new();
                        let _ = reader.read_to_end(&mut buf).await.unwrap();
                        let _ = stream.write_all(buf.as_slice()).await;
                    }
                    else {
                        let _ = req.handle_static(stream, server.clone()).await;
                    }                    
                    break;
                }
            }
        });
    }
}

