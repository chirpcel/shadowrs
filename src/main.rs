mod cli;
mod config;
mod oci;
mod namespace;

use clap::Parser;
use cli::Cli;
use namespace::ContainerNamespace;

fn main() {
    let cli = Cli::try_parse();
    match cli {
        Ok(cli) => {
            let namespaces = get_namespaces(&cli.container_id).unwrap();
            println!("{:?}", namespaces);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn get_namespaces(container_id: &str) -> Result<ContainerNamespace, Box<dyn std::error::Error>> {
    let oci = oci::get_oci();
    let pid = oci.get_pid_by_container_id(container_id);
    match pid {
        Ok(pid) => {
            let namespaces = oci.get_namespaces_by_pid(&pid)?;
            Ok(ContainerNamespace::new(container_id.to_string(), namespaces))
        }
        Err(e) => Err(e),
    }
}


