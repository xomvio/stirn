use std::{fs::File, io::{Read, Write}};
use flate2::{write::GzEncoder, Compression};
pub mod stream;
pub mod utils;
use utils::get_header;

pub const RESPONSE_200:&str = "HTTP/1.1 200 OK\r\n";
pub const RESPONSE_201:&str = "HTTP/1.1 201 Created\r\n";
pub const RESPONSE_404:&str = "HTTP/1.1 404 Not Found\r\n";

pub struct Request {
	pub method: String,
	pub endpoint: String,
	pub protocol: String,
	pub headers: Vec<String>
}

pub struct Response {
    pub headers: String,
    pub body: Vec<u8>,
}

#[derive(Clone)]
pub struct Route {
    pub endpoint: String,
    pub layout: String,
    pub file: String,
}

pub struct HttpBuilder {
    pub port: u16,
    pub routes: Vec<Route>,
    pub error_page: Route,
}

pub struct XomElement {
    key: String,
    val: String,
}

impl XomElement {
    fn new() -> XomElement{
        XomElement { key: "".to_string(), val: "".to_string() }
    }
}

enum Reading {
    None,
    Start,
    Key,
    Val,
    End,
}

pub fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

pub fn initialize_raw(route:Route) -> String {
    let mut file = File::open("files/".to_string() + route.file.as_str()).unwrap();
    let mut filestr: String = String::new();
    file.read_to_string(&mut filestr).unwrap();
    filestr
}

pub fn initialize(route:Route, request: &Request) -> String {
    let newfilestr:String;

    let mut file = File::open("files/".to_string() + route.file.as_str()).unwrap();
    let mut filestr: String = String::new();
    file.read_to_string(&mut filestr).unwrap();

    let mut elements:Vec<XomElement> = vec![];

    if route.layout != "" {
        let mut layoutfile = File::open("files/".to_string() + route.layout.as_str()).unwrap();
        let mut layoutstr: String = String::new();
        layoutfile.read_to_string(&mut layoutstr).unwrap();
        newfilestr = process_commands(layoutstr, filestr, &request);
    }
    else {
        newfilestr = process_commands(filestr, "".to_string(), &request);
    }

    /*let mut elements:Vec<XomElement> = vec![];
    let mut newelem:XomElement = XomElement { key: "".to_string(), val: "".to_string() };*/
        
    newfilestr
}

fn process_commands(filestr: String, nextfilestr: String, request: &Request) -> String {
    let mut elements:Vec<XomElement> = vec![];
    let mut newelem:XomElement = XomElement { key: "".to_string(), val: "".to_string() };
    let mut newfilestr:String = "".to_string();
    let mut read: Reading = Reading::None;

    for ch in filestr.chars() {
        match read {
            Reading::None=>{
                match ch {
                    '@'=>{
                        //newfilestr.push(ch);
                        read = Reading::Start;
                    }
                    _=>newfilestr.push(ch),
                }
            },
            Reading::Start=>{
                match ch {
                    '@'=>{
                        newfilestr.push(ch);
                        read = Reading::None;
                    }
                    _=>{
                        newelem.key.push(ch);
                        read = Reading::Key;
                    }
                }
            },
            Reading::Key=>{
                match ch {
                    '('=>read = Reading::Val,
                    //'@'=>read = Reading::End,
                    _=>newelem.key.push(ch)
                }
            },
            Reading::Val=>{
                match ch {
                    ')'=>read = Reading::End,
                    _=>newelem.val.push(ch),
                }
            },
            Reading::End=>{
                newelem = XomElement{ key:newelem.key.trim().to_string(), val:newelem.val.trim().to_string() };

                match newelem.key.trim() {
                    "callbody"=>{
                        newfilestr.push_str(
                            process_commands(nextfilestr.clone(), "".to_string(), request).as_str()
                        );
                    },
                    "getheader"=>{
                        match get_header(&request.headers, &newelem.val) {
                            Some(headerval)=>{newfilestr.push_str(&headerval)},
                            None=>println!("Warning: User-Agent not found."),
                        }
                    },
                    _=>{},
                }
                
                println!("{}: {}",newelem.key, newelem.val);
                elements.push(newelem);
                read = Reading::None;
                newelem = XomElement{ key:"".to_string(), val:"".to_string() };
            },
        }
    }

    newfilestr
}
