use std::process::Command;

use super::Oci;

pub struct DockerCli {}
impl Oci for DockerCli {
    fn get_pid_by_container_id(&self, container_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("docker")
            .args(&["inspect", "-f", "{{.State.Pid}}", container_id])
            .output()?;
        let pid = String::from_utf8(output.stdout)?;
        Ok(pid.trim().to_string())
    }
    
    fn get_namespaces_by_pid(&self, pid: &str) -> Result<Vec<crate::namespace::Namespace>, Box<dyn std::error::Error>> {
        let mnt_arg = format!("/proc/{}/ns:/mnt", pid);
        let ipc_output = Command::new("docker")
            .args(&["run", "--rm", "-v", &mnt_arg, "busybox", "readlink", "/mnt/ipc"])
            .output()?;
        let ipc = String::from_utf8(ipc_output.stdout)?;
        let mnt_output = Command::new("docker")
            .args(&["run", "--rm", "-v", &mnt_arg, "busybox", "readlink", "/mnt/mnt"])
            .output()?;
        let mnt = String::from_utf8(mnt_output.stdout)?;
        let net_output = Command::new("docker")
            .args(&["run", "--rm", "-v", &mnt_arg, "busybox", "readlink", "/mnt/net"])
            .output()?;
        let net = String::from_utf8(net_output.stdout)?;
        let pid_output = Command::new("docker")
            .args(&["run", "--rm", "-v", &mnt_arg, "busybox", "readlink", "/mnt/pid"])
            .output()?;
        let pid = String::from_utf8(pid_output.stdout)?;
        Ok(vec![
            crate::namespace::Namespace::from_string(&ipc.trim().to_string()).unwrap(),
            crate::namespace::Namespace::from_string(&mnt.trim().to_string()).unwrap(),
            crate::namespace::Namespace::from_string(&net.trim().to_string()).unwrap(),
            crate::namespace::Namespace::from_string(&pid.trim().to_string()).unwrap(),
        ])
    }
}
