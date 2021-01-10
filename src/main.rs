mod client;
mod config;
mod usage;

use actix_web::{App, HttpServer, Responder, get, error, Error, HttpResponse, http::header};
use actix_web::web::Data;
use serde::Serialize;
use actix_cors::Cors;
use tera::Tera;

use crate::client::KubernetesClient;
use crate::config::bind_address;

#[derive(Serialize)]
struct NamespacesResponse {
    namespaces: Vec<client::Namespace>
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("Kubernetes dashboard started");

    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();

        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::CONTENT_TYPE])
                    .max_age(3600)
            )
            .data(tera)
            .service(healthz)
            .service(api_namespaces)
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

#[get("/api/v1/namespaces")]
async fn api_namespaces() -> impl Responder {
    HttpResponse::Ok().json(NamespacesResponse {
        namespaces: KubernetesClient::new().await.container_resources().await
    })
}

#[get("/")]
async fn dashboard_index(tera: Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("namespaces", "Nikita");

    tera.render("index.html", &ctx)
        .map(|v| HttpResponse::Ok().body(v))
        .map_err(|_| error::ErrorInternalServerError("templating_error"))
}