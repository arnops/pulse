use crate::error::{PulseError, Result};
use crate::types::MonitorConfig;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Root configuration structure
#[derive(Debug, Deserialize)]
pub struct Config {
    pub monitors: Vec<MonitorConfig>,
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| PulseError::Config(format!("Failed to read config file: {}", e)))?;

        let config: Config = toml::from_str(&content)?;

        // Validate configuration
        config.validate()?;

        Ok(config)
    }

    /// Validate the configuration
    fn validate(&self) -> Result<()> {
        if self.monitors.is_empty() {
            return Err(PulseError::Config(
                "No monitors defined in configuration".to_string(),
            ));
        }

        for (idx, monitor) in self.monitors.iter().enumerate() {
            if monitor.name.is_empty() {
                return Err(PulseError::Config(format!(
                    "Monitor {} has empty name",
                    idx
                )));
            }

            if monitor.command.is_empty() {
                return Err(PulseError::Config(format!(
                    "Monitor '{}' has empty command",
                    monitor.name
                )));
            }

            if monitor.webhook.is_empty() {
                return Err(PulseError::Config(format!(
                    "Monitor '{}' has empty webhook URL",
                    monitor.name
                )));
            }

            if monitor.interval == 0 {
                return Err(PulseError::Config(format!(
                    "Monitor '{}' has invalid interval (must be > 0)",
                    monitor.name
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_monitors() {
        let config = Config {
            monitors: vec![],
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_interval() {
        let config = Config {
            monitors: vec![MonitorConfig {
                name: "test".to_string(),
                command: "echo test".to_string(),
                interval: 0,
                webhook: "https://example.com".to_string(),
            }],
        };

        assert!(config.validate().is_err());
    }
}
