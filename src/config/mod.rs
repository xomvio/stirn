use std::{collections::HashMap, fs, io::Read};
use xom_json::{self, to_jobject, JArray, JObject, Val};
use crate::utils::log;

use super::Server;

pub struct Config {
    pub port: Option<u16>,
    // DEFAULT SERVER IS NOW INACTIVE AND WILL BE RE-ACTIVATED IN A FUTURE VERSION
    //pub default: Option<String>,
    pub servers: Vec<Server>,
    pub mimetypes: HashMap<String, String>,
    pub default_mimetype: String
}

impl Config {
    pub fn new() -> Config {
        Config { port: None, /*default: Some(String::new()),*/ servers: Vec::new(), mimetypes: HashMap::new(), default_mimetype: String::new() }
    }
}

trait ConfigJObject {
    fn get_port(&mut self) -> Option<u16>;
    //fn get_default(&mut self, config: &JObject);
    fn get_server(&mut self) -> Server;
    fn get_servers(&mut self) -> Vec<Server>;    
    fn get_mimetypes(&mut self) -> HashMap<String, String>;
    fn get_default_mimetype(&mut self) -> String;
}

impl ConfigJObject for JObject {
    fn get_port(&mut self) -> Option<u16> {
        match self.get("port") {
            Some(port_val) => {
                if !port_val.is_number() { panic!("Port must be a number"); }
                let port = port_val.as_u16().unwrap();
                Some(port)
            },
            None => Some(80)
        }
    }
    //fn get_default(&mut self, config: &JObject) {
    //    self.default = get_default(config);
    //}
    fn get_server(&mut self) -> Server {
        let server = Server {
            name: match self.get("name") {
                Some(name) => name.as_string().expect("Server name must be a valid string"),
                None => String::new()
            },
            hostname: self.get("hostname")
                .expect("Server must have a hostname").as_string()
                .expect("Server hostname must be a valid string"),
            port: match self.get("port") {
                Some(port) => port.as_u16().expect("Server port must be a valid u16 number"),
                None => 0
            },
            dir: match self.get("dir") {
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

    fn get_default_mimetype(&mut self) -> String {        
        if let Some(default_mimetype) = self.get("default-mimetype") {
            if !default_mimetype.is_string() { panic!("Default mimetype must be a string"); }
            default_mimetype.as_string().unwrap().to_string()
        } 
        else {
            "application/octet-stream".to_string()
        }
    }

    fn get_mimetypes(&mut self) -> HashMap<String, String> {
        let mut mimetypes:HashMap<String, String> = HashMap::from(
            [ ("aac".to_string(), "audio/aac".to_string()), ("abw".to_string(), "application/x-abiword".to_string()), ("apng".to_string(), "image/apng".to_string()), ("arc".to_string(), "application/x-freearc".to_string()), ("avif".to_string(), "image/avif".to_string()), ("avi".to_string(), "video/x-msvideo".to_string()), ("bin".to_string(), "application/octet-stream".to_string()), ("bmp".to_string(), "image/bmp".to_string()), ("bz".to_string(), "application/x-bzip".to_string()), ("bz2".to_string(), "application/x-bzip2".to_string()), ("c".to_string(), "text/plain".to_string()), ("cab".to_string(), "application/vnd.ms-cab-compressed".to_string()), ("css".to_string(), "text/css".to_string()), ("csv".to_string(), "text/csv".to_string()), ("doc".to_string(), "application/msword".to_string()), ("docx".to_string(), "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string()), ("eot".to_string(), "application/vnd.ms-fontobject".to_string()), ("exe".to_string(), "application/octet-stream".to_string()), ("gif".to_string(), "image/gif".to_string()), ("gz".to_string(), "application/gzip".to_string()), ("htm".to_string(), "text/html".to_string()), ("html".to_string(), "text/html".to_string()), ("ico".to_string(), "image/x-icon".to_string()), ("ics".to_string(), "text/calendar".to_string()), ("jar".to_string(), "application/java-archive".to_string()), ("jpeg".to_string(), "image/jpeg".to_string()), ("jpg".to_string(), "image/jpeg".to_string()), ("js".to_string(), "application/javascript".to_string()), ("json".to_string(), "application/json".to_string()), ("jsonld".to_string(), "application/ld+json".to_string()), ("mid".to_string(), "audio/midi".to_string()), ("midi".to_string(), "audio/midi".to_string()), ("mjs".to_string(), "application/javascript".to_string()), ("mp3".to_string(), "audio/mpeg".to_string()), ("mp4".to_string(), "video/mp4".to_string()), ("mpeg".to_string(), "video/mpeg".to_string()), ("mpkg".to_string(), "application/vnd.apple.installer+xml".to_string()), ("odp".to_string(), "application/vnd.oasis.opendocument.presentation".to_string()), ("ods".to_string(), "application/vnd.oasis.opendocument.spreadsheet".to_string()), ("odt".to_string(), "application/vnd.oasis.opendocument.text".to_string()), ("oga".to_string(), "audio/ogg".to_string()), ("ogv".to_string(), "video/ogg".to_string()), ("ogx".to_string(), "application/ogg".to_string()), ("opus".to_string(), "audio/opus".to_string()), ("otf".to_string(), "font/otf".to_string()), ("png".to_string(), "image/png".to_string()), ("pdf".to_string(), "application/pdf".to_string()), ("php".to_string(), "application/x-httpd-php".to_string()), ("ppt".to_string(), "application/vnd.ms-powerpoint".to_string()), ("pptx".to_string(), "application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string()), ("rar".to_string(), "application/vnd.rar".to_string()), ("rtf".to_string(), "application/rtf".to_string()), ("sh".to_string(), "application/x-sh".to_string()), ("svg".to_string(), "image/svg+xml".to_string()), ("swf".to_string(), "application/x-shockwave-flash".to_string()), ("tar".to_string(), "application/x-tar".to_string()), ("tif".to_string(), "image/tiff".to_string()), ("tiff".to_string(), "image/tiff".to_string()), ("ts".to_string(), "video/mp2t".to_string()), ("ttf".to_string(), "font/ttf".to_string()), ("txt".to_string(), "text/plain".to_string()), ("wav".to_string(), "audio/wav".to_string()), ("webm".to_string(), "video/webm".to_string()), ("webp".to_string(), "image/webp".to_string()), ("woff".to_string(), "font/woff".to_string()), ("woff2".to_string(), "font/woff2".to_string()), ("xhtml".to_string(), "application/xhtml+xml".to_string()), ("xls".to_string(), "application/vnd.ms-excel".to_string()), ("xlsx".to_string(), "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string()), ("xml".to_string(), "application/xml".to_string()), ("zip".to_string(), "application/zip".to_string()) ]
        );

        if let Some(mimetypes_j) = self.get("mimetypes") {
            log(format!("Info: Custom mimetypes found in config. Updating default.").as_str());
            if let Val::Object(mimetypes_j) = mimetypes_j {
                for (key, value) in mimetypes_j.iter() {
                    if !value.is_string() { panic!("Error: Mimetype values must be a string"); }                    
                    mimetypes.insert(key.to_owned(), value.as_string().unwrap().to_string());   
                }
            } else {
                panic!("Error: Mimetypes must be an object");
            }
        }
        
        mimetypes
    }

    fn get_servers(&mut self) -> Vec<Server> {        
        if let Some(servers) = self.get("servers") {
            if let Val::Array(server_j) = servers {
                let mut servers = Vec::new();
                for server_j in server_j.iter() {
                    match server_j.to_owned() { Val::Object(mut server_j) => {
                        servers.push(server_j.get_server());
                        },
                        _ => { panic!("Server must be an object"); }
                    }
                }            
                servers
            } else {
                panic!("Servers must be an array");
            }
        } else {
            panic!("No servers found in config");
        }

    }
}

pub fn get_config() -> Config {    
    let mut jtext = String::new();
    fs::File::open("stirners.json").unwrap().read_to_string(&mut jtext).unwrap();

    let mut config: Config = Config::new();
    
    match xom_json::to_jobject(jtext) { 
        Ok(mut config_j) => {
            config.port = config_j.get_port();
            //config.default = get_default(&config_j);
            config.servers = config_j.get_servers();
            config.default_mimetype = config_j.get_default_mimetype();
            config.mimetypes = config_j.get_mimetypes();
        }
        Err(e) => { panic!("Error: {}", e); }
    }

    config
}
