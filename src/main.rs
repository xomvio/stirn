use xom_http::utils::{HttpBuilder, Route, XomElement};
pub mod pages;

fn main() {
	let routes:Vec<Route> = vec![
		Route{
			endpoint: "/css/bootstrap.min.css".to_string(), 
			layout: "".to_string(), 
			file: "css/bootstrap.min.css".to_string(),
			commands: vec![],
		},
		Route{
			endpoint: "/css/style.css".to_string(), 
			layout: "".to_string(), 
			file: "css/style.css".to_string(),
			commands: vec![],
		},
		Route{
			endpoint: "/".to_string(), 
			layout: "design.rshtml".to_string(), 
			file: "index.rshtml".to_string(),
			commands: pages::index(),
		},
		Route{
			endpoint: "/index".to_string(), 
			layout: "design.rshtml".to_string(), 
			file: "index.rshtml".to_string(),
			commands: pages::index(),
		},
		Route{
			endpoint: "/slm".to_string(), 
			layout: "".to_string(), 
			file:"index.rshtml".to_string(),
			commands: vec![],
		}
	];

	let httpbuilder = HttpBuilder { 
		port: 4221, 
		routes,
		error_page: Route { endpoint: "/error".to_string(), layout: "design.html".to_string(), file: "error.html".to_string(), commands: vec![] },
	};
    xom_http::run(httpbuilder);
}



