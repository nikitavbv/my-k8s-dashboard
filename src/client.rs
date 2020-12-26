use kube::{Resource, Api};
use kube::api::{ListParams, ObjectList};
use serde::{Deserialize, Serialize};

use k8s_openapi::api::core::v1::Pod as KubePod;

use crate::usage::{PodMetrics, pod_metrics};

// container requests and limits and metrics
#[derive(Debug)]
pub struct Namespace {
    pub name: String,
    pub pods: Vec<Pod>,
}

#[derive(Debug)]
pub struct Pod {
    pub name: String,
    pub containers: Vec<Container>,
}

#[derive(Debug)]
pub struct Container {
    pub name: String,

    pub usage: PodContainerMetrics,
    pub requests: PodContainerMetrics,
    pub limits: PodContainerMetrics,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PodContainerMetrics {
    pub cpu: String,
    pub memory: String,
}

// kube client
pub struct KubernetesClient {
    client: kube::Client
}

impl KubernetesClient {

    pub async fn new() -> Self {
        KubernetesClient {
            client: kube::Client::try_default().await.unwrap()
        }
    }

    pub async fn container_resources(&self) -> Vec<Namespace> {
        let pods = &self.pods().await;
        let pod_metrics = &self.container_metrics().await;

        let namespaces = pods.iter()
            .map(|v| v.metadata.namespace.clone().unwrap_or("".to_string()))
            .chain(pod_metrics.iter().map(|v| v.metadata.namespace.clone()))
            .map(|name| Namespace {
                name,
                pods: Vec::new()
            })
            .collect();

        namespaces
    }

    async fn pods(&self) -> Vec<KubePod> {
        let pod_api: Api<KubePod> = Api::all(self.client.clone());
        pod_api.list(&ListParams::default()).await.unwrap().items
    }

    async fn container_metrics(&self) -> Vec<PodMetrics> {
        pod_metrics(&self.client).await
    }
}