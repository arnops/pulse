use crate::discord::DiscordClient;
use crate::error::{PulseError, Result};
use crate::types::MonitorConfig;
use std::sync::Arc;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

/// Monitor that executes commands and tracks output changes
pub struct Monitor {
    config: MonitorConfig,
    discord_client: Arc<DiscordClient>,
    last_output: Option<String>,
}

impl Monitor {
    /// Create a new monitor with the given configuration
    pub fn new(config: MonitorConfig, discord_client: Arc<DiscordClient>) -> Self {
        Self {
            config,
            discord_client,
            last_output: None,
        }
    }

    /// Start the monitoring loop (runs indefinitely)
    pub async fn run(mut self) -> Result<()> {
        println!(
            "🚀 Starting monitor '{}' (interval: {}s)",
            self.config.name, self.config.interval
        );

        // Send initial message
        self.discord_client
            .send_message(
                &self.config.webhook,
                &format!("✅ Monitor '{}' started", self.config.name),
            )
            .await?;

        loop {
            match self.execute_and_check().await {
                Ok(changed) => {
                    if changed {
                        println!(
                            "📊 [{}] Change detected, notification sent",
                            self.config.name
                        );
                    }
                }
                Err(e) => {
                    eprintln!("❌ [{}] Error: {}", self.config.name, e);
                }
            }

            sleep(Duration::from_secs(self.config.interval)).await;
        }
    }

    /// Execute the command and check if output has changed
    async fn execute_and_check(&mut self) -> Result<bool> {
        let output = self.execute_command().await?;

        // Check if output has changed
        let has_changed = match &self.last_output {
            None => true, // First execution
            Some(last) => last != &output,
        };

        if has_changed {
            // Send notification to Discord
            self.discord_client
                .send_embed(&self.config.webhook, &self.config.name, &output)
                .await?;

            // Update last output
            self.last_output = Some(output);
        }

        Ok(has_changed)
    }

    /// Execute the command and return its output
    async fn execute_command(&self) -> Result<String> {
        // Parse command and arguments (simple space-based split)
        let parts: Vec<&str> = self.config.command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(PulseError::CommandExecution("Empty command".to_string()));
        }

        let cmd = parts[0];
        let args = &parts[1..];

        // Execute command
        let output = Command::new(cmd).args(args).output().await.map_err(|e| {
            PulseError::CommandExecution(format!("Failed to execute '{}': {}", cmd, e))
        })?;

        // Combine stdout and stderr
        let mut result = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() {
            if !result.is_empty() {
                result.push_str("\n\n--- stderr ---\n");
            }
            result.push_str(&stderr);
        }

        // Include exit status if non-zero
        if !output.status.success() {
            result.push_str(&format!(
                "\n\n[Exit code: {}]",
                output.status.code().unwrap_or(-1)
            ));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_echo_command() {
        let config = MonitorConfig {
            name: "test".to_string(),
            command: "echo hello".to_string(),
            interval: 1,
            webhook: "https://example.com".to_string(),
        };

        let discord_client = Arc::new(DiscordClient::new().unwrap());
        let monitor = Monitor::new(config, discord_client);

        let output = monitor.execute_command().await.unwrap();
        assert!(output.contains("hello"));
    }
}
