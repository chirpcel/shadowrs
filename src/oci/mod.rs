mod docker_cli;
use crate::namespace::Namespace;

pub trait Oci {
    fn get_pid_by_container_id(&self, container_id: &str) -> Result<String, Box<dyn std::error::Error>>;
    fn get_namespaces_by_pid(&self, pid: &str) -> Result<Vec<Namespace>, Box<dyn std::error::Error>>;
}

pub fn get_oci() -> Box<dyn Oci> {
    Box::new(docker_cli::DockerCli {})
}
