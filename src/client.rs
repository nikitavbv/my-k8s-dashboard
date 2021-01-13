use kube::Api;
use kube::api::ListParams;
use serde::Serialize;

use k8s_openapi::api::core::v1::Pod as KubeAPIPod;
use k8s_openapi::api::core::v1::Container as KubeAPIContainer;

use crate::usage::{PodMetrics, pod_metrics, PodMetricsContainer};
use std::collections::HashSet;

// container requests and limits and metrics
#[derive(Debug, Serialize)]
pub struct Namespace {
    pub name: String,
    pub pods: Vec<Pod>,
}

#[derive(Debug, Serialize)]
pub struct Pod {
    pub name: String,
    pub containers: Vec<Container>,
}

#[derive(Debug, Serialize)]
pub struct Container {
    pub name: String,

    pub usage: Option<ResourceMetrics>,
    pub requests: Option<ResourceMetrics>,
    pub limits: Option<ResourceMetrics>,
}

#[derive(Debug, Serialize)]
pub struct ResourceMetrics {
    cpu: u64,
    memory: u64,
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
            .collect::<HashSet<String>>().iter()
            .map(|name| Namespace {
                name: name.clone(),
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
        let resources_containers: Vec<(String, KubeAPIContainer)> = resources.iter()
            .filter_map(|v| v.spec.clone().map(|s| (v.metadata.name.clone().unwrap_or("".to_string()), s.containers)))
            .flat_map(|v| v.clone().1.iter().map(|s| (v.0.clone(), s.clone())).collect::<Vec<(String, KubeAPIContainer)>>())
            .map(|v| v.clone())
            .collect();
        let usage_containers: Vec<(String, PodMetricsContainer)> = usage.iter()
            .flat_map(|v| v.containers.iter().map(|c| (v.metadata.name.clone(), c.clone())).collect::<Vec<(String, PodMetricsContainer)>>())
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

    fn filter_resources_containers_by_pod(containers: &Vec<(String, KubeAPIContainer)>, pod: &str) -> Vec<KubeAPIContainer> {
        containers.iter()
            .filter(|v| v.0 == pod)
            .map(|v| v.1.clone())
            .collect()
    }

    fn filter_usage_containers_by_pod(containers: &Vec<(String, PodMetricsContainer)>, pod: &str) -> Vec<PodMetricsContainer> {
        containers.iter()
            .filter(|v| v.0 == pod)
            .map(|v| v.1.clone())
            .collect()
    }

    fn combine_container_resources_and_usage(resources: &Vec<KubeAPIContainer>, usage: &Vec<PodMetricsContainer>) -> Vec<Container> {
        let containers = resources.iter()
            .map(|v| v.name.clone())
            .chain(usage.iter().map(|v| v.name.clone()))
            .map(|name| Container {
                name: name.clone(),
                usage: usage.iter()
                    .filter(|v| v.name == name)
                    .map(|v| ResourceMetrics {
                        cpu: Self::parse_cpu_usage(&v.usage.cpu),
                        memory: Self::parse_memory_usage(&v.usage.memory),
                    })
                    .next(),
                requests: resources.iter()
                    .filter(|v| v.name == name)
                    .map(|v| v.resources.clone())
                    .next()
                    .flatten()
                    .map(|v| v.requests)
                    .flatten()
                    .map(|v| ResourceMetrics {
                        cpu: v.get("cpu").map(|v| Self::parse_cpu_usage(&v.0)).unwrap_or(0),
                        memory: v.get("memory").map(|v| Self::parse_memory_usage(&v.0)).unwrap_or(0),
                    }),
                limits: resources.iter()
                    .filter(|v| v.name == name)
                    .map(|v| v.resources.clone())
                    .next()
                    .flatten()
                    .map(|v| v.requests)
                    .flatten()
                    .map(|v| ResourceMetrics {
                        cpu: v.get("cpu").map(|v| Self::parse_cpu_usage(&v.0)).unwrap_or(0),
                        memory: v.get("memory").map(|v| Self::parse_memory_usage(&v.0)).unwrap_or(0),
                    })
            })
            .collect();

        containers
    }

    fn parse_cpu_usage(cpu: &str) -> u64 {
        if cpu == "0" {
            return 0
        } else if cpu.ends_with("n") {
            cpu.replace("n", "").parse().expect("failed to parse cpu usage in nanocpus")
        } else if cpu.ends_with("u") {
            Self::cpu_micros_to_nanos(cpu.replace("u", "").parse().expect("failed to parse cpu usage in microcpus"))
        } else if cpu.ends_with("m") {
            Self::cpu_millis_to_nanos(cpu.replace("m", "").parse().expect("failed to parse cpu usage in millicpus"))
        } else {
            // panic is bad, but it is ok for now
            panic!("can't parse cpu usage: {}", cpu);
            0
        }
    }

    fn parse_memory_usage(memory: &str) -> u64 {
        if memory.ends_with("Ki") {
            memory.replace("Ki", "").parse().expect("failed to parse memory in Ki")
        } else if memory.ends_with("M") {
            Self::mebibyte_to_kilobyte(memory.replace("M", "").parse().expect("failed to parse memory in M"))
        } else if memory.ends_with("Mi") {
            Self::megabyte_to_kilobyte(memory.replace("Mi", "").parse().expect("failed to parse memory usage in Mi"))
        } else {
            // panic is bad, but it is ok for now
            panic!("can't parse memory usage: {}", memory);
            0
        }
    }

    fn cpu_millis_to_nanos(nanos: u64) -> u64 {
        nanos * 1000000
    }

    fn cpu_micros_to_nanos(micros: u64) -> u64 {
        micros * 1000
    }

    fn mebibyte_to_kilobyte(megabyte: u64) -> u64 {
        megabyte * 1049 // APIs use M as n = 1000
    }

    fn megabyte_to_kilobyte(megabyte: u64) -> u64 {
        megabyte * 1024
    }

    async fn pods(&self) -> Vec<KubeAPIPod> {
        let pod_api: Api<KubeAPIPod> = Api::all(self.client.clone());
        pod_api.list(&ListParams::default()).await.unwrap().items
    }

    async fn container_metrics(&self) -> Vec<PodMetrics> {
        pod_metrics(&self.client).await
    }
}
