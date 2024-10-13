use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub provider: Option<String>,
    pub persistent: bool,
    pub docker: DockerConfig,
    pub nix: NixConfig,
    pub nixery: NixeryConfig,
}

impl Config {
    pub fn with_tools(tools: String) -> Self {
        Config {
            provider: Some(String::from("nixery")),
            persistent: bool::default(),
            docker: DockerConfig {
                command: default_command()
            },
            nix: NixConfig {
                image: default_nix_image()
            },
            nixery: NixeryConfig {
                registry: default_registry(),
                tools: Some(tools),
            },
            
        }
    }

    pub fn with_provider(provider: String) -> Self {
        Config {
            provider: Some(provider),
            persistent: bool::default(),
            docker: DockerConfig {
                command: default_command()
            },
            nix: NixConfig {
                image: default_nix_image()
            },
            nixery: NixeryConfig {
                registry: default_registry(),
                tools: None,
            },
            
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            provider: Some(String::from("busybox")),
            persistent: bool::default(),
            docker: DockerConfig {
                command: default_command()
            },
            nix: NixConfig {
                image: default_nix_image()
            },
            nixery: NixeryConfig {
                registry: default_registry(),
                tools: None,
            },
        }
    }
}

#[derive(Deserialize)]
pub struct NixConfig {
    #[serde(default = "default_nix_image")]
    pub image: String
}

fn default_nix_image() -> String {
    "nixos/nix".to_string()
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
