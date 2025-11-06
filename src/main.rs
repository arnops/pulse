use clap::{Parser, Subcommand};
use pulse::{Config, DiscordClient, Monitor, MonitorConfig};
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "pulse")]
#[command(about = "High-performance CLI monitoring tool", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a single monitor
    Run {
        /// Command to execute
        #[arg(short, long)]
        command: String,

        /// Interval in seconds between executions
        #[arg(short, long)]
        interval: u64,

        /// Discord webhook URL
        #[arg(short, long)]
        webhook: String,

        /// Optional name for the monitor (defaults to command)
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Start multiple monitors from a config file
    Start {
        /// Path to the config file
        #[arg(short, long)]
        config: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            command,
            interval,
            webhook,
            name,
        } => {
            let monitor_name = name.unwrap_or_else(|| command.clone());
            let config = MonitorConfig {
                name: monitor_name,
                command,
                interval,
                webhook,
            };

            run_single_monitor(config).await?;
        }
        Commands::Start { config } => {
            run_from_config(&config).await?;
        }
    }

    Ok(())
}

/// Run a single monitor
async fn run_single_monitor(config: MonitorConfig) -> anyhow::Result<()> {
    println!("🎯 Pulse - High-Performance Monitoring");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let discord_client = Arc::new(DiscordClient::new()?);
    let monitor = Monitor::new(config, discord_client);

    monitor.run().await?;

    Ok(())
}

/// Run multiple monitors from config file
async fn run_from_config(config_path: &str) -> anyhow::Result<()> {
    println!("🎯 Pulse - High-Performance Monitoring");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Loading config from: {}\n", config_path);

    let config = Config::from_file(config_path)?;
    let discord_client = Arc::new(DiscordClient::new()?);

    println!("✅ Loaded {} monitor(s)\n", config.monitors.len());

    // Spawn a task for each monitor
    let mut tasks = vec![];

    for monitor_config in config.monitors {
        let client = Arc::clone(&discord_client);
        let monitor = Monitor::new(monitor_config, client);

        let task = tokio::spawn(async move {
            if let Err(e) = monitor.run().await {
                eprintln!("Monitor failed: {}", e);
            }
        });

        tasks.push(task);
    }

    // Wait for all monitors (they run indefinitely)
    for task in tasks {
        task.await?;
    }

    Ok(())
}
