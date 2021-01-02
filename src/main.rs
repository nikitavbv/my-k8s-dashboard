mod client;
mod config;
mod usage;

use actix_web::{App, HttpServer, Responder, get, error, Error, HttpResponse};
use actix_web::web::Data;
use tera::Tera;
use crate::client::KubernetesClient;
use crate::config::bind_address;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,kube=debug");
    env_logger::init();
    println!("Hello, world!");

    let client = KubernetesClient::new().await;
    client.container_resources().await;
    println!("res is {:?}", client.container_resources().await);

    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();

        App::new()
            .data(tera)
            .service(healthz)
            .service(dashboard_index)
    })
        .bind(bind_address())?
        .run()
        .await
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    "ok"
}

#[get("/")]
async fn dashboard_index(tera: Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("word", "Nikita");
    tera.render("index.html", &ctx)
        .map(|v| HttpResponse::Ok().body(v))
        .map_err(|_| error::ErrorInternalServerError("templating_error"))
}