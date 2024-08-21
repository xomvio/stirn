mod libx;
pub use libx::{strict::StrictServer, Server, request, get_hostname, handle};

pub fn start_strict(server: StrictServer) {
    server.run();
}

/*pub fn start(server: Server) {
    server.run();
}*/