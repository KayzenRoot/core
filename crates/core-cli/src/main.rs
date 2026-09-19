use clap::{Parser, Subcommand};
use core_config::{ConfigOverrides, CoreConfig};
use core_contracts::SchemaVersion;
use core_runtime::Supervisor;
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(name = "core", version, about = "NexLabs CORE headless runtime")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Start,
    Status,
    Validate,
    Doctor,
    Version,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> ExitCode {
    match run(Cli::parse()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

async fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::Version => print_json(
            serde_json::json!({ "schema": SchemaVersion::CURRENT, "name": "core", "version": env!("CARGO_PKG_VERSION") }),
        ),
        Command::Validate => {
            let config = CoreConfig::from_sources(None, None, [], &ConfigOverrides::default(), 1)
                .map_err(|e| e.to_string())?;
            print_json(config.redacted_diagnostics())
        }
        Command::Doctor => print_json(
            serde_json::json!({ "schema": SchemaVersion::CURRENT, "headless": true, "rust_runtime": "stable", "tokio": true, "hive_dependency": false, "llm_calls": 0 }),
        ),
        Command::Start => {
            let config = CoreConfig::from_sources(None, None, [], &ConfigOverrides::default(), 1)
                .map_err(|e| e.to_string())?;
            let mut supervisor = Supervisor::new(config).map_err(|e| e.to_string())?;
            let receipt = supervisor.bootstrap().await.map_err(|e| e.to_string())?;
            print_json(serde_json::to_value(receipt).map_err(|e| e.to_string())?)
        }
        Command::Status => {
            let config = CoreConfig::from_sources(None, None, [], &ConfigOverrides::default(), 1)
                .map_err(|e| e.to_string())?;
            let supervisor = Supervisor::new(config).map_err(|e| e.to_string())?;
            print_json(serde_json::to_value(supervisor.status()).map_err(|e| e.to_string())?)
        }
    }
    Ok(())
}

fn print_json(value: serde_json::Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("JSON serialization is infallible")
    );
}
