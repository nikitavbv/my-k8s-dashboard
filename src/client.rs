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
        let resources = self.pods().await;
        let usage = self.container_metrics().await;

        let namespaces: Vec<Namespace> = resources.iter()
            .map(|v| v.metadata.namespace.clone().unwrap_or("".to_string()))
            .chain(usage.iter().map(|v| v.metadata.namespace.clone()))
            .map(|name| Namespace {
                name,
                pods: Vec::new()
            })
            .collect();

        let namespaces: Vec<Namespace> = namespaces.iter()
            .map(|namespace| Namespace {
                name: namespace.name.clone(),
                pods: Self::combine_resources_and_usage(
                    &Self::filter_resources_by_namespace(&resources, &namespace.name),
                    &Self::filter_usage_by_namespace(&usage, &namespace.name)
                )
            })
            .collect();

        namespaces
    }

    fn filter_resources_by_namespace(resources: &Vec<KubePod>, namespace: &str) -> Vec<KubePod> {
        resources.iter()
            .filter(|v| &v.metadata.namespace.clone().unwrap_or("".to_string()) == namespace)
            .map(|v| v.clone())
            .collect()
    }

    fn filter_usage_by_namespace(usage: &Vec<PodMetrics>, namespace: &str) -> Vec<PodMetrics> {
        usage.iter()
            .filter(|v| &v.metadata.namespace == namespace)
            .map(|v| v.clone())
            .collect()
    }

    fn combine_resources_and_usage(resources: &Vec<KubePod>, usage: &Vec<PodMetrics>) -> Vec<Pod> {
        // TODO: finish this
        Vec::new()
    }

    async fn pods(&self) -> Vec<KubePod> {
        let pod_api: Api<KubePod> = Api::all(self.client.clone());
        pod_api.list(&ListParams::default()).await.unwrap().items
    }

    async fn container_metrics(&self) -> Vec<PodMetrics> {
        pod_metrics(&self.client).await
    }
}