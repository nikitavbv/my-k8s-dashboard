use serde::{Deserialize, Serialize};

use kube::Resource;
use kube::api::{ListParams, ObjectList};
use schemars::JsonSchema;
use kube_derive::CustomResource;

use crate::client::PodContainerMetrics;

// pod metrics - kube api
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
    pub usage: PodContainerMetrics,
}

pub async fn pod_metrics(client: &kube::Client) -> Vec<PodMetrics> {
    // note: this is a hack, I hope to change it to something better once I have better understanding of kube client
    let resource = Resource::all::<Pod>();
    let req = resource.list(&ListParams::default()).unwrap();
    let res = client.request::<ObjectList<PodMetrics>>(req).await.unwrap();

    res.items
}