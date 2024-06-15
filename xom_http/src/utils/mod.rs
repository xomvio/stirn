use std::{fs::File, io::{Read, Write}, net::TcpStream};
pub mod stream;
pub mod common;
use common::get_header;

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

pub struct HttpBuilder {
    pub port: u16,
    pub routes: Vec<Route>,
    pub error_page: Route,
}

#[derive(Clone)]
pub struct XomElement {
    pub key: String,
    pub val: String,
}

impl XomElement {
    pub fn new() -> XomElement{
        XomElement { key: "".to_string(), val: "".to_string() }
    }
    pub fn new_vec() -> Vec<XomElement> {
        vec![]
    }
}

enum Reading {
    None,
    Start,
    Key,
    Val,
    End,
}

#[derive(Clone)]
pub struct Route {
    pub endpoint: String,
    pub layout: String,
    pub file: String,
    pub commands: Vec<XomElement>
}

impl Route {
    pub fn init(self, req: &Request, commands: Vec<XomElement>) -> Response {
        let mut resp:Response = Response { headers: String::new(), body: vec![] };
        resp.headers = format!("{}\r\npage not found: {}", RESPONSE_404.to_string(), req.endpoint);
    
        let filestr;
			
        let file_extension = self.file.split('.').last().unwrap_or("txt");
        let content_type = match file_extension {
            "css" => "text/css",
            "ico" => "image/x-icon",
            "rshtml" | "html" => "text/html",
            "txt" | "text" => "text/plain",
            "js" => "text/javascript",
            "json" => "application/json",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "woff" => "font/woff",
            _ => "text/plain",
        };
        resp.headers = format!("{RESPONSE_200}Content-Type: {content_type}\r\n");

        filestr = match file_extension {
            "rshtml"=> initialize(self, &req, commands),
            _ => initialize_raw(self),
        };

        //check gzip encoding
        match get_header(&req.headers, "Accept-Encoding") {
            Some(headerval)=>{ 
                if headerval.contains("gzip") {	//gzip compress
                    resp.body = common::compress_data(filestr.as_bytes());	//gzipping
                    resp.headers += "Content-Encoding: gzip\r\n";						
                }
                else {	//unsupported compress
                    resp.body = filestr.as_bytes().to_vec();
                }
            },
            None=>resp.body = filestr.as_bytes().to_vec(), //no compress
        }

        resp.headers += &("Content-Length: ".to_string() + &resp.body.len().to_string() + "\r\n\r\n");
        println!("{}",resp.headers);
        resp
    }
}

pub fn stream_write(mut streamx:TcpStream, resp:Response) -> TcpStream{

		streamx.write_all(resp.headers.as_bytes()).unwrap();
		streamx.write_all(&resp.body).unwrap();
		streamx.flush().unwrap();
        streamx
}

pub fn initialize_raw(route:Route) -> String {    
    let mut file = File::open("files/".to_string() + route.file.as_str()).unwrap();
    let mut filestr: String = String::new();
    file.read_to_string(&mut filestr).unwrap();
    filestr
}

pub fn initialize(route:Route, request: &Request, mut commands:Vec<XomElement>) -> String {
    let newfilestr:String;

    let mut file = File::open("files/".to_string() + route.file.as_str()).unwrap();
    let mut filestr: String = String::new();
    file.read_to_string(&mut filestr).unwrap();

    if route.layout != "" {
        let mut layoutfile = File::open("files/".to_string() + route.layout.as_str()).unwrap();
        let mut layoutstr: String = String::new();
        layoutfile.read_to_string(&mut layoutstr).unwrap();
        (newfilestr, commands) = process_commands(layoutstr, filestr, &request, commands);
    }
    else {
        (newfilestr, commands) = process_commands(filestr, "".to_string(), &request, commands);
    }

    /*let mut elements:Vec<XomElement> = vec![];
    let mut newelem:XomElement = XomElement { key: "".to_string(), val: "".to_string() };*/
        
    newfilestr
}

fn process_commands(filestr: String, nextfilestr: String, request: &Request, mut commands:Vec<XomElement>) -> (String, Vec<XomElement>) {
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
                let mut add_cmd = false;

                match newelem.key.trim() {
                    "callbody"=>{
                        let body;
                        (body, commands) = process_commands(nextfilestr.clone(), "".to_string(), request, commands);
                        newfilestr.push_str(&body);
                    },
                    "getheader"=>{
                        match get_header(&request.headers, &newelem.val) {
                            Some(headerval)=>{newfilestr.push_str(&headerval)},
                            None=>println!("Warning: User-Agent not found."),
                        }
                    },
                    "get"=>{
                        for cmd in &commands {
                            if cmd.key == newelem.val {
                                newfilestr.push_str(&cmd.val);
                                break;
                            }
                        }
                    },
                    _=>{
                        //if already exists
                        let mut exists = false;
                        for i in 0..commands.len() {    //writing here like this, otherwise we need to clone and reassign all commands.
                            if commands[i].key == newelem.key {
                                commands[i].val = newelem.val.to_owned();
                                exists = true;
                            }
                        }
                        if !exists {
                            commands.push(newelem.clone());
                        }
                    },
                }
                
                println!("{}: {}",newelem.key, newelem.val);
                if add_cmd {  }
                read = Reading::None;
                newelem = XomElement::new();
            },
        }
    }

    for cm in &commands {
        println!("{} x {}", cm.key, cm.val);
    }

    (newfilestr, commands)
}
