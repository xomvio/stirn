
pub fn get_header(lines:&Vec<String>, header:&str) -> Option<String> { //example key "Accept-Encoding"
    for l in lines {
        let mut name_and_val = l.split(": ");
        if name_and_val.next().expect("broken header key") == header {
            return Some(name_and_val.next().expect("broken header value").to_string());
        }
    }
    return None
}