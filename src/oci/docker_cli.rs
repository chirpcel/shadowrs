use std::{process::Command, thread::sleep, time::Duration};
use crate::config::Config;

use super::{ContainerInfo, Oci};

pub struct DockerCli {
    config: Config,
}
impl Oci for DockerCli {

    fn new(config: crate::config::Config) -> Self {
        DockerCli { config }
    }

    fn get_container_info_byid(&self, container_id: &str) -> Result<ContainerInfo, Box<dyn std::error::Error>> {
        let output = Command::new(self.config.docker.command.as_str())
            .args(&["inspect", "-f", "{{.State.Pid}}", container_id])
            .output()?;
        let pid = String::from_utf8(output.stdout)?;
        Ok(ContainerInfo {
            container_id: container_id.to_string(), 
            container_pid: pid.trim().to_string(),
        })
    }

    fn prepare_shadow_container(&self) -> Result<(), Box<dyn std::error::Error>> {
        sleep(Duration::from_secs(3));
        Ok(())
    }
    
    fn run_shadow_container(&self, container_info: ContainerInfo) -> Result<(), Box<dyn std::error::Error>> {
        let shadow_image = get_shadow_image_name(&self.config.nixery.registry, &self.config.nixery.tools.clone().unwrap_or_default());
        println!("Shadowing container {} with image: {}", container_info.container_id, shadow_image);
        Command::new(self.config.docker.command.as_str())
            .arg("run")
            .arg("-it")
            .arg("--rm")
            .args(&["--pid", format!("container:{}", container_info.container_id).as_str()])
            .args(&["--network", format!("container:{}", container_info.container_id).as_str()])
            .arg(shadow_image)
            .arg("bash")
            .spawn()?.wait()?;
        Ok(())
    }
}

#[cfg(target_arch = "aarch64")]
fn get_shadow_image_name(reg: &str, tools: &str) -> String {
    format!("{}/shell/arm64{}", reg, tools)
}

#[cfg(target_arch = "x86_64")]
fn get_shadow_image_name(reg: &str, tools: &str) -> String {
    format!("{}/shell{}", reg, tools)
}
