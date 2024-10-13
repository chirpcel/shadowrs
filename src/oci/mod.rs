use crate::config::Config;

mod docker_cli;

#[derive(Debug)]
pub struct ContainerInfo {
    pub container_id: String,
    pub container_pid: String,
}

pub trait Oci {
    fn new(config: Config) -> Self where Self: Sized;
    fn get_container_info_byid(&self, container_id: &str) -> Result<ContainerInfo, Box<dyn std::error::Error>>;
    fn prepare_shadow_container(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn run_shadow_container(&self, container_info: ContainerInfo) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn get_oci(config: Config) -> Box<dyn Oci> {
    Box::new(docker_cli::DockerCli::new(config))
}
