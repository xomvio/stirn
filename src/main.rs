use xom_http;
use xom_http::rapid::{HttpBuilder, Route};

fn main() {
	let routes:Vec<Route> = vec![
		Route{endpoint: "/".to_string(), layout: "design.html".to_string(), file: "index.html".to_string()},
		Route{endpoint: "/index".to_string(), layout: "design.html".to_string(), file: "index.html".to_string()},
		Route{endpoint: "/slm".to_string(), layout: "".to_string(), file:"index.html".to_string()},
	];
	let httpbuilder = HttpBuilder { 
		port: 4221, 
		routes: routes
	};
    xom_http::run(httpbuilder);
}
