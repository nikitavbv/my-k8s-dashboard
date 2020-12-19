use kube::{Api, Client};
use k8s_openapi::api::core::v1::{Pod, Node};

use kube::api::{ListParams, Meta};

#[actix_rt::main]
async fn main() {
    println!("Hello, world!");

    let client = Client::try_default().await.unwrap();

    let pods: Api<Pod> = Api::all(client);

    for pod in pods.list(&ListParams::default()).await.unwrap() {
        let pod_status = match &pod.status {
            Some(v) => v,
            None => {
                println!("no status for pod {}", pod.name());
                continue;
            }
        };

        for container in pod_status.clone().container_statuses.unwrap_or(Vec::new()) {
            //let state = container.state.unwrap();
            //println!("container status: {:?}", &container);
        }

        println!("pod!!! {:?}", pod);
    }
}
