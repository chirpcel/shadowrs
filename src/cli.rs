use clap::{builder::PossibleValue, Parser, ValueEnum};

/// shadowrs - Command that shadow existing container with additional tools
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    
    #[clap(short, long, default_value = "busybox")]
    pub provider: ShadowrsProvider,

    /// A '/' seprated list of nix packages to be added to the nixery image
    #[clap(short, long)]
    pub tools: Option<String>,

    /// Container id or name to be shadowed
    #[arg()]
    pub container_id: String,

}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum ShadowrsProvider {
    busybox,
    nix,
    nixery
}

impl ValueEnum for ShadowrsProvider {
    fn value_variants<'a>() -> &'a [Self] {
        &[ShadowrsProvider::busybox, ShadowrsProvider::nix, ShadowrsProvider::nixery]
    }
    
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            ShadowrsProvider::busybox => PossibleValue::new("busybox"),
            ShadowrsProvider::nix => PossibleValue::new("nix"),
            ShadowrsProvider::nixery => PossibleValue::new("nixery"),
        })
    }
}
