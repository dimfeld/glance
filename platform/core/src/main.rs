//! The core of Glance
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use error_stack::{Report, ResultExt};
use glance_core::{error::Error, tracing_config, Platform, PlatformOptions};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Serve(ServeCommand),
}

#[derive(Args, Debug)]
struct ServeCommand {
    #[clap(long, env = "GLANCE_BASE_DIR")]
    base_dir: Option<PathBuf>,

    #[clap(long = "db", env = "GLANCE_DATABASE_URL")]
    database_url: String,

    #[clap(long, env = "GLANCE_HOST", default_value_t = String::from("127.0.0.1"))]
    host: String,

    #[clap(long, env = "GLANCE_PORT", default_value_t = 6749)]
    port: u16,

    #[clap(long = "env", env = "GLANCE_ENV", default_value_t = String::from("development"))]
    env: String,
}

async fn serve(cmd: ServeCommand) -> Result<(), Report<Error>> {
    // TODO ability to configure trace export
    glance_core::tracing_config::configure(tracing_config::TracingExportConfig::None)
        .change_context(Error::ServerStart)?;

    let platform = Platform::new(PlatformOptions {
        base_dir: cmd.base_dir,
        database_url: Some(cmd.database_url),
    })
    .await;

    let server = glance_core::server::create_server(glance_core::server::Config {
        env: "development".into(),
        host: cmd.host,
        port: cmd.port,
        db: platform.db.clone(),
    })
    .await?;

    server.run().await?;

    platform.shutdown().await;

    // Wait for all the remaining traces to export
    tokio::task::spawn_blocking(|| glance_core::tracing_config::teardown())
        .await
        .change_context(Error::Shutdown)?;

    Ok(())
}

/// The entrypoint
#[tokio::main]
pub async fn main() -> Result<(), Report<Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Serve(cmd) => serve(cmd).await,
    }
}
