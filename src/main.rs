pub mod client;

use kube::{Client, Resource};

use kube::api::{ListParams, ObjectList};

use crate::client::KubernetesClient;

#[actix_rt::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug,kube=debug");
    env_logger::init();
    println!("Hello, world!");

    let client = KubernetesClient::new().await;
    println!("res is {:?}", client.pod_metrics().await);
}
