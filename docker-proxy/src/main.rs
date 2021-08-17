use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    println!("Hello, docker proxy!");

    let client_key = thrussh_keys::key::KeyPair::generate_ed25519().unwrap();
    let client_pubkey = Arc::new(client_key.clone_public_key());
    let mut config = thrussh::server::Config::default();
    config.connection_timeout = Some(std::time::Duration::from_secs(3));
    config.auth_rejection_time = std::time::Duration::from_secs(3);
    config.keys.push(thrussh_keys::key::KeyPair::generate_ed25519().unwrap());
    let config = Arc::new(config);
    let sh = Server {
        client_pubkey,
        clients: Arc::new(Mutex::new(HashMap::new())),
        id: 0,
    };

    tokio::time::timeout(
        std::time::Duration::from_secs(1),
    thrussh::server::run(config, "0.0.0.0:2222", sh)
    );
}

#[derive(Clone)]
struct Server {
}