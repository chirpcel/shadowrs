mod cli;
mod config;
mod oci;

use clap::Parser;
use cli::Cli;
use config::Config;
use spinners::{Spinner, Spinners};

fn main() {
    let cli = Cli::try_parse();
    match cli {
        Ok(cli) => {
            if(cli.tools.is_some()) {
                let config = Config::with_tools(cli.tools.unwrap());
                spawn_shadow_container(&cli.container_id, config);
            } else {
                println!("No tools specified, using default tools");
                let config = Config::default();
                spawn_shadow_container(&cli.container_id, config);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn spawn_shadow_container(container_id: &str, config: Config) {
    let mut collect_info = Spinner::new(Spinners::Dots, "Collecting target container info".into());
    let oci = oci::get_oci(config);
    let container_info = oci.get_container_info_byid(container_id);
    collect_info.stop_with_message("✅ Collected target container info".into());
    let mut prepare_shadow = Spinner::new(Spinners::Dots, "Preparing shadow container".into());
    oci.prepare_shadow_container().unwrap();
    prepare_shadow.stop_with_message("✅ Prepared shadow container".into());
    oci.run_shadow_container(container_info.unwrap()).unwrap();
}


