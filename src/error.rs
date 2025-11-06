use thiserror::Error;

#[derive(Error, Debug)]
pub enum PulseError {
    #[error("Failed to execute command: {0}")]
    CommandExecution(String),

    #[error("Failed to send Discord webhook: {0}")]
    DiscordWebhook(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, PulseError>;
