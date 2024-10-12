use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub nixery: NixeryConfig,
    pub docker: DockerConfig
}

impl Config {
    pub fn with_tools(tools: String) -> Self {
        Config {
            nixery: NixeryConfig {
                registry: default_registry(),
                tools: Some(tools),
            },
            docker: DockerConfig {
                command: default_command()
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            nixery: NixeryConfig {
                registry: default_registry(),
                tools: None,
            },
            docker: DockerConfig {
                command: default_command()
            }
        }
    }
}

#[derive(Deserialize)]
pub struct NixeryConfig {
    #[serde(default = "default_registry")]
    pub registry: String,
    pub tools: Option<String>
}

fn default_registry() -> String {
    "nixery.dev".to_string()
}

#[derive(Deserialize)]
pub struct DockerConfig {
    #[serde(default = "default_command")]
    pub command: String
}

fn default_command() -> String {
    "docker".to_string()
}
