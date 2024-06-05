use std::{fs::File, io::Read};
use flate2;

pub const RESPONSE_404:&str = "HTTP/1.1 404 Not Found\r\n";
pub const RESPONSE_200:&str = "HTTP/1.1 200 OK\r\n";
pub const RESPONSE_201:&str = "HTTP/1.1 201 Created\r\n";
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
}

#[derive(Debug)]
pub struct XomElement {
    key: String,
    val: String,
}

enum Reading {
    None,
    Start,
    Key,
    Val,
    End,
}

pub fn get_header(lines:&Vec<String>, key:&str) -> String { //example key "Accept-Encoding"
    for l in lines {
        let mut name_and_val = l.split(": ");
        if name_and_val.next().expect("broken header key") == key {
            return name_and_val.next().expect("broken header value").to_string();
        }
    }
    return "undefined".to_string()
}

pub fn initialize(route:Route) -> String {
    let newfilestr:String;    

    let mut file = File::open("files/".to_string() + route.file.as_str()).unwrap();
    let mut filestr: String = String::new();
    file.read_to_string(&mut filestr).unwrap();

    if route.layout != "" {
        let mut layoutfile = File::open("files/".to_string() + route.layout.as_str()).unwrap();
        let mut layoutstr: String = String::new();
        layoutfile.read_to_string(&mut layoutstr).unwrap();
        newfilestr = process_commands(layoutstr, filestr);
    }
    else {
        newfilestr = process_commands(filestr, "".to_string());
    }

    /*let mut elements:Vec<XomElement> = vec![];
    let mut newelem:XomElement = XomElement { key: "".to_string(), val: "".to_string() };*/
        
    println!("{newfilestr}");
    newfilestr
}

fn process_commands(filestr: String, nextfilestr: String) -> String {
    let mut elements:Vec<XomElement> = vec![];
    let mut newelem:XomElement = XomElement { key: "".to_string(), val: "".to_string() };
    let mut newfilestr:String = "".to_string();
    let mut read: Reading = Reading::None;

    for ch in filestr.chars() {
        match read {
            Reading::None=>{
                match ch {
                    '<'=>{
                        newfilestr.push(ch);
                        read = Reading::Start;
                    }
                    _=>newfilestr.push(ch),
                }
            },
            Reading::Start=>{
                match ch {
                    '@'=>{
                        newfilestr.pop();
                        read = Reading::Key;
                    }
                    _=>{
                        newfilestr.push(ch);
                        read = Reading::None;
                    }
                }
            },
            Reading::Key=>{
                match ch {
                    '='=>read = Reading::Val,
                    '@'=>read = Reading::End,
                    _=>newelem.key.push(ch)
                }
            },
            Reading::Val=>{
                match ch {
                    '@'=>read = Reading::End,
                    _=>newelem.val.push(ch),
                }
            },
            Reading::End=>{
                match ch {
                    '>'=>{
                        newelem = XomElement{ key:newelem.key.trim().to_string(), val:newelem.val.trim().to_string() };

                        match newelem.key.trim() {
                            "CallBody"=>{
                                newfilestr.push_str(
                                    process_commands(nextfilestr.clone(), "".to_string()).as_str()
                                );
                            },
                            _=>{},
                        }
                        
                        elements.push(newelem);
                        read = Reading::None;
                        newelem = XomElement{ key:"".to_string(), val:"".to_string() };
                    },
                    _=>panic!("Error reading Xom Element. > expected"),
                }
            },
        }
    }

    newfilestr
}