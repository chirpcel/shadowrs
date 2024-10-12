use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    nixery: NixeryConfig,
    docker: DockerConfig
}

impl Default for Config {
    fn default() -> Self {
        Config {
            nixery: NixeryConfig {
                registry: default_registry()
            },
            docker: DockerConfig {
                command: default_command()
            }
        }
    }
}

#[derive(Deserialize)]
struct NixeryConfig {
    #[serde(default = "default_registry")]
    registry: String
}

fn default_registry() -> String {
    "nixery.dev".to_string()
}

#[derive(Deserialize)]
struct DockerConfig {
    #[serde(default = "default_command")]
    command: String
}

fn default_command() -> String {
    "docker".to_string()
}
