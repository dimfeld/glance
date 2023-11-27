use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(long, env)]
    pub base_dir: Option<PathBuf>,
}

#[tokio::main]
pub async fn main() {}
