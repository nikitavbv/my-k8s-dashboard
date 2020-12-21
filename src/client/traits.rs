pub trait KubernetesClient {
    fn new() -> Self where Self: Sized;
}