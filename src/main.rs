mod cli;
mod config;
mod oci;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::try_parse();
    match cli {
        Ok(cli) => {
            println!("{:?}", cli);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
