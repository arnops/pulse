use crate::error::{PulseError, Result};
use crate::types::{DiscordEmbed, DiscordMessage, EmbedField};
use reqwest::Client;

/// Discord webhook client with connection pooling
pub struct DiscordClient {
    client: Client,
}

impl DiscordClient {
    /// Create a new Discord client with optimized settings
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .pool_max_idle_per_host(10)
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self { client })
    }

    /// Send a simple text message to a Discord webhook
    pub async fn send_message(&self, webhook_url: &str, content: &str) -> Result<()> {
        let message = DiscordMessage {
            content: content.to_string(),
            embeds: None,
        };

        self.send(&message, webhook_url).await
    }

    /// Send a message with embed to a Discord webhook
    pub async fn send_embed(
        &self,
        webhook_url: &str,
        monitor_name: &str,
        output: &str,
    ) -> Result<()> {
        let truncated_output = if output.len() > 4000 {
            format!("{}...\n(truncated)", &output[..4000])
        } else {
            output.to_string()
        };

        let embed = DiscordEmbed {
            title: Some(format!("📊 Monitor: {}", monitor_name)),
            description: None,
            color: Some(0x5865F2), // Discord blurple
            fields: Some(vec![EmbedField {
                name: "Output".to_string(),
                value: format!("```\n{}\n```", truncated_output),
                inline: Some(false),
            }]),
        };

        let message = DiscordMessage {
            content: format!("🔔 **Change detected in {}**", monitor_name),
            embeds: Some(vec![embed]),
        };

        self.send(&message, webhook_url).await
    }

    /// Internal method to send the message via HTTP POST
    async fn send(&self, message: &DiscordMessage, webhook_url: &str) -> Result<()> {
        let response = self
            .client
            .post(webhook_url)
            .json(message)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(PulseError::DiscordWebhook(format!(
                "HTTP {}: {}",
                status, error_text
            )));
        }

        Ok(())
    }
}

impl Default for DiscordClient {
    fn default() -> Self {
        Self::new().expect("Failed to create Discord client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = DiscordClient::new();
        assert!(client.is_ok());
    }
}
