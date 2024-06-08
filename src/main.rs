use xom_http::rapid::{HttpBuilder, Route};

fn main() {
	let routes:Vec<Route> = vec![
		Route{endpoint: "/css/bootstrap.min.css".to_string(), layout: "".to_string(), file: "css/bootstrap.min.css".to_string()},
		Route{endpoint: "/css/style.css".to_string(), layout: "".to_string(), file: "css/style.css".to_string()},
		Route{endpoint: "/".to_string(), layout: "design.html".to_string(), file: "index.html".to_string()},
		Route{endpoint: "/index".to_string(), layout: "design.html".to_string(), file: "index.html".to_string()},
		Route{endpoint: "/slm".to_string(), layout: "".to_string(), file:"index.html".to_string()},
	];
	let httpbuilder = HttpBuilder { 
		port: 4221, 
		routes,
		error_page: Route { endpoint: "/error".to_string(), layout: "design.html".to_string(), file: "error.html".to_string() }
	};
    xom_http::run(httpbuilder);
}
