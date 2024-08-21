use super::Request;
use std::net::TcpStream;
use itertools::Itertools;
use std::io::{BufReader, Read};

impl Request {
    pub fn get_header(&self, header: &str) -> Option<&str> {
        for line in self.headers.iter() {
            let mut name_and_val = line.split(": ");
            if name_and_val.next().expect("broken header key") == header {
                return Some(name_and_val.next().expect("broken header value"));
            }
        }
        return None;
    }

    pub fn is_gzip_accepted(&self) -> bool {
        return self.get_header("Accept-Encoding").is_some_and(|headerval| headerval.contains("gzip"));
    }

    pub fn read(mut stream:&TcpStream) -> Request {
        
        let mut reader = BufReader::new(&mut stream);   //tcp reader

        let mut reading_buffer = [0; 1024];	//reading buffer

        let _ = reader.read(&mut reading_buffer).expect("cannot read stream for buffer");	//writing stream to buffer as bytes

        let buffer_str = String::from_utf8_lossy(&reading_buffer);	//convert buffer to string

        let buffer_lines:Vec<String> = buffer_str.split("\r\n").collect_vec().iter().map(|&s| s.into()).collect();	//headers
        let first_line:Vec<String> = buffer_lines[0].split_whitespace().collect_vec().iter().map(|&s| s.into()).collect();	//first line of header

        let req = Request {
            method: first_line[0].to_string(),
            endpoint: first_line[1].to_string(),
            protocol:first_line[2].to_string(), 
            headers:  buffer_lines[1..].to_vec(),
            //nekot: "".to_string(),
        };
        
        /*let req = if first_line[1].contains('?') 
            {   
                Request {
                    method: first_line[0].to_string(),
                    endpoint: first_line[1].to_string().split('?').collect_vec()[0].to_string(),
                    protocol:first_line[2].to_string(), 
                    headers:  buffer_lines[1..].to_vec(),
                    //nekot: first_line[1].to_string().split('?').collect_vec()[1].to_string(),
                }
            }
            else {
                Request {
                    method: first_line[0].to_string(),
                    endpoint: first_line[1].to_string(),
                    protocol:first_line[2].to_string(), 
                    headers:  buffer_lines[1..].to_vec(),
                    //nekot: "".to_string(),
                }
            };*/
        req
    }
}
