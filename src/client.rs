use kube::{Resource, Api};
use kube::api::{ListParams, ObjectList};
use serde::{Deserialize, Serialize};

use k8s_openapi::api::core::v1::Pod as KubeAPIPod;
use k8s_openapi::api::core::v1::Container as KubeAPIContainer;

use crate::usage::{PodMetrics, pod_metrics, PodMetricsContainer};

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
                pods: Self::combine_pod_resources_and_usage(
                    &Self::filter_resources_by_namespace(&resources, &namespace.name),
                    &Self::filter_usage_by_namespace(&usage, &namespace.name)
                )
            })
            .collect();

        namespaces
    }

    fn filter_resources_by_namespace(resources: &Vec<KubeAPIPod>, namespace: &str) -> Vec<KubeAPIPod> {
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

    fn combine_pod_resources_and_usage(resources: &Vec<KubeAPIPod>, usage: &Vec<PodMetrics>) -> Vec<Pod> {
        let resources_containers: Vec<KubeAPIContainer> = resources.iter()
            .filter_map(|v| v.spec.clone().map(|v| v.containers))
            .flat_map(|v| v)
            .map(|v| v.clone())
            .collect();
        let usage_containers: Vec<PodMetricsContainer> = usage.iter()
            .flat_map(|v| v.containers.iter())
            .map(|v| v.clone())
            .collect();

        let pods = resources.iter()
            .filter_map(|v| v.metadata.name.clone())
            .chain(usage.iter().map(|v| v.metadata.name.clone()))
            .map(|name| Pod {
                name: name.clone(),
                containers: Self::combine_container_resources_and_usage(
                    &Self::filter_resources_containers_by_pod(&resources_containers, &name),
                    &Self::filter_usage_containers_by_pod(&usage_containers, &name)
                )
            })
            .collect();

        pods
    }

    fn filter_resources_containers_by_pod(containers: &Vec<KubeAPIContainer>, pod: &str) -> Vec<KubeAPIContainer> {
        // TODO: implement this
        containers.clone()
    }

    fn filter_usage_containers_by_pod(containers: &Vec<PodMetricsContainer>, pod: &str) -> Vec<PodMetricsContainer> {
        // TODO: implement this
        containers.clone()
    }

    fn combine_container_resources_and_usage(resources: &Vec<KubeAPIContainer>, usage: &Vec<PodMetricsContainer>) -> Vec<Container> {
        // TODO: implement this
        Vec::new()
    }

    async fn pods(&self) -> Vec<KubeAPIPod> {
        let pod_api: Api<KubeAPIPod> = Api::all(self.client.clone());
        pod_api.list(&ListParams::default()).await.unwrap().items
    }

    async fn container_metrics(&self) -> Vec<PodMetrics> {
        pod_metrics(&self.client).await
    }
}