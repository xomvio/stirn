
use xom_http::{self, utils::{self, common::get_header, XomElement}};

pub fn index() -> Vec<XomElement> {
    vec![
        XomElement {key: "key".to_string(), val:"val".to_string()},
        XomElement {key: "name".to_string(), val:get_name()},
    ]
}

fn get_name() -> String {
    "yasir".to_string()
}
