use std::{fs, io::Read};
use xom_json::{self, to_jobject, JArray, JObject, Val};
use crate::utils::log;

use super::Server;

pub struct Config {
    pub port: Option<u16>,
    // DEFAULT SERVER IS NOW INACTIVE AND WILL BE RE-ACTIVATED IN A FUTURE VERSION
    //pub default: Option<String>,
    pub servers: Vec<Server>,
}

impl Config {
    pub fn new() -> Config {
        Config { port: None, /*default: Some(String::new()),*/ servers: Vec::new() }
    }
}

pub fn get_config() -> Config {
    let mut jtext = String::new();
    fs::File::open("stirners.json").unwrap().read_to_string(&mut jtext).unwrap();

    let mut config: Config = Config::new();
    match to_jobject(jtext) { 
        Ok(config_j) => {

            config.port = get_port(&config_j);
            //config.default = get_default(&config_j);

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

fn get_port(config: &JObject) -> Option<u16> {
    match config.get("port") {
        Some(port_val) => {
            if !port_val.is_number() { panic!("Port must be a number"); }
            let port = port_val.as_u16().unwrap();
            Some(port)
        },
        None => Some(80)
    }
}

// DEFAULT SERVER IS NOW INACTIVE AND WILL BE RE-ACTIVATED IN A FUTURE VERSION
/*fn get_default(config: &JObject) -> String {
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
}*/

fn get_server(serverj: &JObject) -> Server {
    let server = Server {
        name: match serverj.get("name") {
            Some(name) => name.as_string().expect("Server name must be a valid string"),
            None => String::new()
        },
        hostname: serverj.get("hostname")
            .expect("Server must have a hostname").as_string()
            .expect("Server hostname must be a valid string"),
        port: match serverj.get("port") {
            Some(port) => port.as_u16().expect("Server port must be a valid u16 number"),
            None => 0
        },
        dir: match serverj.get("dir") {
            Some(dir) => {
                dir.as_string().expect("Server dir must be a valid string")
            },
            None => String::new()
        }
    };

    if server.port != 0 && !server.dir.is_empty() {
        log( format!("port and dir cannot be used together. dir will be ignored for {}", server.hostname).as_str());
    }

    server
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
