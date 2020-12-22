use kube::Resource;
use kube::api::{ListParams, ObjectList};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube_derive::CustomResource;

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

pub struct KubernetesClient {
    client: kube::Client
}

impl KubernetesClient {

    pub async fn new() -> Self {
        KubernetesClient {
            client: kube::Client::try_default().await.unwrap()
        }
    }

    pub async fn pod_metrics(&self) -> Vec<PodMetrics> {
        // note: this is a hack, I hope to change it to something better once I have better understanding of kube client
        let resource = Resource::all::<Pod>();
        let req = resource.list(&ListParams::default()).unwrap();
        let res = self.client.request::<ObjectList<PodMetrics>>(req).await.unwrap();

        res.items
    }
}