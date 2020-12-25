mod client;
mod config;
mod usage;

use actix_web::{App, HttpServer, Responder, get};
use crate::client::KubernetesClient;
use crate::config::bind_address;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,kube=debug");
    env_logger::init();
    println!("Hello, world!");

    let client = KubernetesClient::new().await;
    println!("res is {:?}", client.container_resources().await);

    HttpServer::new(|| App::new()
            .service(healthz)
        )
        .bind(bind_address())?
        .run()
        .await
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    "ok"
}