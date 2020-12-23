use std::env::var;

// http server
pub fn bind_address() -> String {
    var("BIND_ADDRESS").unwrap_or("0.0.0.0:8080".into())
}