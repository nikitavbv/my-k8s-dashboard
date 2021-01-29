use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::Mutex;

use actix_rt::{spawn, time};

use crate::client::KubernetesClient;

#[derive(Copy, Clone)]
struct MonitoringEntry {
    total_cpu: u128,
    total_memory: u128,

    prev_cpu: u64,
    prev_memory: u64,

    updated_at: u64,
}

lazy_static! {
    static ref MONITORING_DATA: Mutex<HashMap<String, MonitoringEntry>> = Mutex::new(HashMap::new());
}

pub fn start_monitoring() {
    println!("starting monitoring job...");

    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60));

        loop {
            interval.tick().await;
            run_monitoring_iteration().await;
        }
    });
}

async fn run_monitoring_iteration() {
    println!("running monitoring iteration");

    let updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let resources = KubernetesClient::new().await.container_resources().await;

    let mut monitoring_stats = MONITORING_DATA.lock().unwrap();
    for namespace in resources {
        for pod in namespace.pods {
            for container in pod.containers {
                if container.usage.is_none() {
                    continue;
                }

                let key = make_monitoring_data_key(&namespace.name, &pod.name, &container.name);
                let usage = container.usage.unwrap();

                let monitoring_entry: Option<MonitoringEntry> = monitoring_stats.get(&key).map(|v| v.clone());

                let cpu_usage = monitoring_entry.map(|v| (usage.cpu + v.prev_cpu) * (updated_at - v.updated_at) / 2).unwrap_or(0) as u128;
                let memory_usage = monitoring_entry.map(|v| (usage.memory + v.prev_memory) * (updated_at - v.updated_at) / 2).unwrap_or(0) as u128;

                let total_cpu = monitoring_entry.map(|v| v.total_cpu).unwrap_or(0) + cpu_usage;
                let total_memory = monitoring_entry.map(|v| v.total_memory).unwrap_or(0) + memory_usage;

                let entry = MonitoringEntry {
                    total_cpu,
                    total_memory,

                    prev_cpu: usage.cpu,
                    prev_memory: usage.memory,

                    updated_at
                };

                monitoring_stats.insert(key, entry);
            }
        }
    }
}

fn make_monitoring_data_key(namespace_name: &str, pod_name: &str, container_name: &str) -> String {
    format!("{}/{}/{}", namespace_name, pod_name, container_name)
}