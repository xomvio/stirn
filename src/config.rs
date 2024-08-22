use std::{fs, io::Read};
use xom_json::{self, to_jobject, JArray, JObject, Val};
//use crate::server::Server;
use super::Server;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub default: String,
    pub servers: Vec<Server>,
}

//pub static mut CONFIG: Config = Config { port: 0, default: String::new(), servers: Vec::new() };

impl Config {
    pub fn new() -> Config {
        Config { port: 0, default: String::new(), servers: Vec::new() }
    }
}

pub fn get_config() -> Config {
    let mut jtext = String::new();
    fs::File::open("stirners.json").unwrap().read_to_string(&mut jtext).unwrap();

    let mut config: Config = Config::new();
    match to_jobject(jtext) { 
        Ok(config_j) => {

            config.port = get_port(&config_j);
            config.default = get_default(&config_j);

            match config_j.get("servers") { 
                Some(servers) => {
                    match servers {
                        Val::Array(servers) => {
                            config.servers = get_servers(servers);
                        },
                        _ => { panic!("Servers must be an array"); } 
                    }
                },
                None => { panic!("No servers found in config"); }
            }
        },
        Err(e) => { panic!("Error: {}", e); }
    }

    config
}

fn get_port(config: &JObject) -> u16 {
    match config.get("port") {
        Some(port) => {
            if !port.is_number() { panic!("Port must be a number"); }
            port.as_u16().unwrap()
        },
        None => { panic!("Error: No Port found in config");}
    }
}

fn get_default(config: &JObject) -> String {
    match config.get("default") {
        Some(default) => {
            if !default.is_string() { panic!("Default must be a string"); }
            default.as_string().unwrap().to_string()
        },
        None => { 
            println!("Warning: No Default found in config"); 
            String::new()
        }
    }
}

fn get_server(server: &JObject) -> Server {
    if server.get("name").is_none() { panic!("Server must have a name"); }
    if server.get("url").is_none() { panic!("Server must have a url"); }
    if server.get("port").is_none() { panic!("Server must have a port"); }
    if server.get("dir").is_none() { panic!("Server must have a dir"); }

    let name = server.get("name").unwrap().as_string().unwrap();
    let url = server.get("url").unwrap().as_string().unwrap();
    let port = server.get("port").unwrap().as_u16().unwrap();
    let dir = server.get("dir").unwrap().as_string().unwrap();

    Server { name, url, port,dir }
}

fn get_servers(servers_j: &JArray) -> Vec<Server> {
    let mut servers = Vec::new();
    for server_j in servers_j.iter() {
        match server_j { Val::Object(server_j) => {
            servers.push(get_server(server_j));
            },
            _ => { panic!("Server must be an object"); }
        }
    }

    servers
}
