use kube::{Client, Resource};
use kube_derive::CustomResource;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use kube::api::{ListParams, ObjectList};

#[derive(CustomResource, Serialize, Deserialize, Default, Clone, Debug, JsonSchema)]
#[kube(
    group = "metrics.k8s.io",
    version = "v1beta1",
    kind="Pod",
    namespaced
)]
pub struct PodMetricsSchema {
}

#[derive(Deserialize, Debug, Clone)]
pub struct PodMetrics {
    pub metadata: PodMetricsMetadata,
    pub timestamp: String,
    pub window: String,
    pub containers: Vec<PodMetricsContainer>
}

#[derive(Deserialize, Debug, Clone)]
pub struct PodMetricsMetadata {
    pub name: String,
    pub namespace: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PodMetricsContainer {
    pub name: String,
    pub usage: PodMetricsContainerUsage,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PodMetricsContainerUsage {
    pub cpu: String,
    pub memory: String,
}

#[actix_rt::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug,kube=debug");
    env_logger::init();
    println!("Hello, world!");

    let client = Client::try_default().await.unwrap();

    // note: this is a hack, I hope to change it to something better once I have better understanding of kube client
    let resource = Resource::all::<Pod>();
    let req = resource.list(&ListParams::default()).unwrap();
    let res = client.request::<ObjectList<PodMetrics>>(req).await.unwrap();

    println!("res is {:?}", res);
}
