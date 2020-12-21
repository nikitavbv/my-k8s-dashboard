use crate::client::traits::KubernetesClient;
use crate::client::client::SimpleKubernetesClient;

pub fn connect() -> Box<dyn KubernetesClient> {
    Box::new(SimpleKubernetesClient::new())
}