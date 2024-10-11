use clap::Parser;

/// shadowrs - Command that shadow existing container with additional tools
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    
    /// A '/' seprated list of nix packages to be added to the container
    #[clap(short, long)]
    pub tools: Option<String>,

    /// Container id or name to be shadowed
    #[arg()]
    pub container_id: String,

}
