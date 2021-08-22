use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs::{File, read};
use std::path::Path;
use thrussh::CryptoVec;
use thrussh::{ChannelId, server, server::{Auth, Session}};
use thrussh_keys::{encode_pkcs8_pem, decode_secret_key};
use log::*;
use env_logger::Env;
use thrussh_keys::key::KeyPair;

const SERVER_PEM_FILE: &str = ".server_key.pem";

/**
I can debug responses using curl:
curl --unix-socket /var/run/docker.sock http://docker.example.com/_ping
*/

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Hello, docker proxy!");

    let client_key = get_keypair();
    let client_pubkey = Arc::new(client_key.clone_public_key());

    let mut config = thrussh::server::Config::default();
    // For some reason it takes connection_timeout for client to connect
    config.connection_timeout = Some(std::time::Duration::from_secs(10));
    config.auth_rejection_time = std::time::Duration::from_secs(30);
    config.keys.push(thrussh_keys::key::KeyPair::generate_ed25519().unwrap());
    let config = Arc::new(config);
    let sh = Server {
        client_pubkey,
        clients: Arc::new(Mutex::new(HashMap::new())),
        id: 0,
    };

    thrussh::server::run(config, "0.0.0.0:2222", sh).await.unwrap();
}

fn get_keypair() -> KeyPair {
    let path = Path::new(SERVER_PEM_FILE);
    if path.exists() {
        info!("loading secret key from file");
        read(path).map(|v| decode_secret_key(&String::from_utf8_lossy(&v), None).unwrap()).unwrap()
    } else {
        info!("generating new secret key");
        let keypair = thrussh_keys::key::KeyPair::generate_ed25519().unwrap();
        let client_file = File::create(path).unwrap();
        encode_pkcs8_pem(&keypair, &client_file).unwrap();
        keypair
    }
}

#[derive(Clone)]
struct Server {
    client_pubkey: Arc<thrussh_keys::key::PublicKey>,
    clients: Arc<Mutex<HashMap<(usize, ChannelId), thrussh::server::Handle>>>,
    id: usize,
}

impl server::Server for Server {
    type Handler = Self;

    fn new(&mut self, _: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        s
    }
}

impl server::Handler for Server {
    type Error = anyhow::Error;
    type FutureAuth = futures::future::Ready<Result<(Self, server::Auth), anyhow::Error>>;
    type FutureUnit = futures::future::Ready<Result<(Self, Session), anyhow::Error>>;
    type FutureBool = futures::future::Ready<Result<(Self, Session, bool), anyhow::Error>>;

    fn finished_auth(mut self, auth: Auth) -> Self::FutureAuth {
        info!("auth finished");

        futures::future::ready(Ok((self, auth)))
    }

    fn finished_bool(self, b: bool, session: server::Session) -> Self::FutureBool {
        futures::future::ready(Ok((self, session, b)))
    }

    fn finished(self, session: server::Session) -> Self::FutureUnit {
        info!("session finished");

        futures::future::ready(Ok((self, session)))
    }

    fn channel_open_direct_tcpip(self, channel: ChannelId, host_to_connect: &str, port_to_connect: u32, originator_address: &str, originator_port: u32, session: Session) -> Self::FutureUnit {
        info!("opening direct tcpip");
        futures::future::ready(Ok((self, session)))
    }

    fn channel_open_session(self, channel: ChannelId, session: server::Session) -> Self::FutureUnit {
        info!("openning session to a new client");
        
        {
            let mut clients = self.clients.lock().unwrap();
            clients.insert((self.id, channel), session.handle());
        }
 
        futures::future::ready(Ok((self, session)))
        // self.finished(session)
    }

    fn auth_publickey(self, user: &str, public_key: &thrussh_keys::key::PublicKey) -> Self::FutureAuth {
        info!("handling auth with publickey");
        self.finished_auth(server::Auth::Accept)
    }

    fn auth_keyboard_interactive(self, user: &str, submethods: &str, response: Option<server::Response>) -> Self::FutureAuth {
        info!("handling auth keyboard interactive");
        self.finished_auth(server::Auth::Accept)
    }

    fn data(self, channel: ChannelId, data: &[u8], mut session: Session) -> Self::FutureUnit {
        // for docker, data seems to contain http request
        info!("got data from client: {}", String::from_utf8_lossy(data));

        // for docker, cryptovec response should contain http response
        session.data(channel, CryptoVec::from_slice("hello world".as_bytes()));

        futures::future::ready(Ok((self, session)))
        //self.finished(session)
    }

    fn extended_data(self, channel: ChannelId, code: u32, data: &[u8], session: Session) -> Self::FutureUnit {
        info!("got extended data");
        futures::future::ready(Ok((self, session)))
    }

    fn pty_request(self, channel: ChannelId, term: &str, col_width: u32, row_height: u32, pix_width: u32, pix_height: u32, modes: &[(thrussh::Pty, u32)], session: Session) -> Self::FutureUnit {
        info!("got pty request");
        futures::future::ready(Ok((self, session)))
    }

    fn shell_request(self, channel: ChannelId, session: Session) -> Self::FutureUnit {
        info!("got shell request");
        futures::future::ready(Ok((self, session)))
    }

    fn exec_request(self, channel: ChannelId, data: &[u8], session: Session) -> Self::FutureUnit {
        info!("got exec request: {}", String::from_utf8_lossy(data));
        futures::future::ready(Ok((self, session)))
    }

    fn tcpip_forward(self, address: &str, port: u32, session: Session) -> Self::FutureBool {
        info!("got tcpip forward");
        futures::future::ready(Ok((self, session, true)))
    }
} 