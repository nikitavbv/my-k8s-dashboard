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

use crate::client::{KubernetesClient, Container};
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
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::CONTENT_TYPE])
                    .max_age(3600)
            )
            .service(healthz)
            .service(api_namespaces)
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
    let kube_client = KubernetesClient::new().await;
    let container_resources = kube_client.container_resources().await;

    let containers_without_limits: Vec<Container> = container_resources.iter()
        .flat_map(|v| v.pods.clone())
        .flat_map(|v| v.containers.clone())
        .filter(|v| v.limits.is_none())
        .collect();

    println!("containers without limits: {:?}", &containers_without_limits);

    let notifications = containers_without_limits.iter()
        .map(|v| format!("There is a container without a limit set: {}", v.name))
        .collect();

    HttpResponse::Ok().json(NamespacesResponse {
        notifications,
        namespaces: container_resources.iter()
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
