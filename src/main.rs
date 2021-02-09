#[macro_use]
extern crate lazy_static;

mod client;
mod config;
mod usage;
mod monitoring;

use actix_web::{App, HttpServer, Responder, get, error, Error, HttpResponse, http::header};
use actix_web::web::Data;
use serde::Serialize;
use actix_cors::Cors;
use tera::Tera;

use crate::client::KubernetesClient;
use crate::config::bind_address;
use crate::monitoring::{start_monitoring, MonitoringEntry, monitoring_data};

#[derive(Serialize)]
struct NamespacesResponse {
    notifications: Vec<String>,
    namespaces: Vec<NamespaceResponse>,
}

#[derive(Serialize)]
struct NamespaceResponse {
    name: String,
    pods: Vec<PodResponse>,
}

#[derive(Serialize)]
struct PodResponse {
    name: String,
    containers: Vec<ContainerResponse>,
}

#[derive(Serialize)]
struct ContainerResponse {
    name: String,

    usage: Option<client::ResourceMetrics>,
    requests: Option<client::ResourceMetrics>,
    limits: Option<client::ResourceMetrics>,

    total_resources: Option<MonitoringEntry>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("Kubernetes dashboard started");

    start_monitoring();

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
        notifications: Vec::new(),
        namespaces: KubernetesClient::new().await.container_resources().await.iter()
            .map(|c| to_namespace_response(&c))
            .collect()
    })
}

fn to_namespace_response(namespace: &client::Namespace) -> NamespaceResponse {
    NamespaceResponse {
        name: namespace.name.clone(),
        pods: namespace.pods.iter().map(|v| to_pod_response(&namespace.name, &v)).collect()
    }
}

fn to_pod_response(namespace_name: &str, pod: &client::Pod) -> PodResponse {
    PodResponse {
        name: pod.name.clone(),
        containers: pod.containers.iter().map(|v| to_container_response(&namespace_name, &pod.name, &v)).collect()
    }
}

fn to_container_response(namespace_name: &str, pod_name: &str, container: &client::Container) -> ContainerResponse {
    ContainerResponse {
        name: container.name.clone(),
        usage: container.usage.clone(),
        requests: container.requests.clone(),
        limits: container.limits.clone(),
        total_resources: monitoring_data(&namespace_name, &pod_name, &container.name)
    }
}

#[get("/")]
async fn dashboard_index(tera: Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("namespaces", "Nikita");

    tera.render("index.html", &ctx)
        .map(|v| HttpResponse::Ok().body(v))
        .map_err(|_| error::ErrorInternalServerError("templating_error"))
}