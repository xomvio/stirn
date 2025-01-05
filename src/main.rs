mod config;
mod utils;
use config::{get_config, Config};
use utils::{response::ResponseBuilder, stream_read, Server, RESPONSE_500};
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
                    req.error = "Host header not found in request".to_string();
                    ResponseBuilder {
                        dir: "".to_string(),
                        endpoint: "".to_string(),
                        is_gzip: false,
                        content_type: "text/html".to_string(),
                        stream,
                        status: RESPONSE_500.to_string(),
                        error: req.error,
                    }
                    .build()
                    .send().await;
                    return;
                }
            };
            for server in CONFIG.servers.iter() {
                if server.hostname == hostname {
                    let _ = req.handle(stream, server.dir.clone()).await;
                    break;
                }
            }
        });
    }
/*    while let Ok((mut stream, _)) = listener.accept() {
        tokio::spawn(async move {
            let mut req= stream_read(&mut stream).await;
            let hostname = match req.get_header("Host") {
                Some(host) => host,
                None => { req.error = "Host header not found in request".to_string();
                    ResponseBuilder { dir: "".to_string(), endpoint: "".to_string(), is_gzip: false, content_type: "text/html".to_string(), stream, status: RESPONSE_500.to_string(), error: req.error }.build().send();
                    return;
                }
            };

            for server in CONFIG.servers.iter() {
                if server.hostname == hostname {
                    let _ = req.handle(stream, server.dir.clone());
                    break;
                }
            }
        });
    }*/
}

