use crate::client::traits::KubernetesClient;

pub struct SimpleKubernetesClient {
}

impl KubernetesClient for SimpleKubernetesClient {

    fn new() -> Self {
        Self {}
    }
}