use serde::{Deserialize, Serialize};

/// Configuration for a single monitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    /// Name/identifier for this monitor
    pub name: String,
    /// Command to execute
    pub command: String,
    /// Interval in seconds between executions
    pub interval: u64,
    /// Discord webhook URL
    pub webhook: String,
}

/// Discord webhook message payload
#[derive(Debug, Serialize)]
pub struct DiscordMessage {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<DiscordEmbed>>,
}

/// Discord embed for rich messages
#[derive(Debug, Serialize)]
pub struct DiscordEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<EmbedField>>,
}

/// Discord embed field
#[derive(Debug, Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}
