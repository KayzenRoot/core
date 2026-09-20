//! Typed configuration with explicit precedence and secret-safe diagnostics.

use core_contracts::RuntimeGeneration;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigSource {
    CompiledDefault,
    Repository,
    Machine,
    Environment,
    Cli,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigProvenance {
    pub source: ConfigSource,
    pub field: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretReference {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreConfig {
    pub journal_path: PathBuf,
    pub max_frame_size: usize,
    pub startup_timeout_ms: u64,
    pub shutdown_timeout_ms: u64,
    pub quality_floor: u8,
    pub require_hive: bool,
    pub secret_reference: Option<SecretReference>,
    pub generation: RuntimeGeneration,
    pub provenance: BTreeMap<String, ConfigProvenance>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    journal_path: Option<PathBuf>,
    max_frame_size: Option<usize>,
    startup_timeout_ms: Option<u64>,
    shutdown_timeout_ms: Option<u64>,
    quality_floor: Option<u8>,
    require_hive: Option<bool>,
    secret_reference: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ConfigOverrides {
    pub journal_path: Option<PathBuf>,
    pub max_frame_size: Option<usize>,
    pub startup_timeout_ms: Option<u64>,
    pub shutdown_timeout_ms: Option<u64>,
    pub quality_floor: Option<u8>,
    pub require_hive: Option<bool>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("TOML configuration is invalid: {0}")]
    Parse(String),
    #[error("configuration validation failed: {0}")]
    Invalid(String),
}

impl CoreConfig {
    pub fn defaults(boot_epoch: u64) -> Self {
        let mut config = Self {
            journal_path: PathBuf::from(".core/runtime.journal"),
            max_frame_size: 1024 * 1024,
            startup_timeout_ms: 30_000,
            shutdown_timeout_ms: 30_000,
            quality_floor: 50,
            require_hive: false,
            secret_reference: None,
            generation: RuntimeGeneration::new(boot_epoch),
            provenance: BTreeMap::new(),
        };
        config.mark_defaults();
        config
    }

    pub fn from_sources(
        repository_toml: Option<&str>,
        machine_toml: Option<&str>,
        env: impl IntoIterator<Item = (String, String)>,
        cli: &ConfigOverrides,
        boot_epoch: u64,
    ) -> Result<Self, ConfigError> {
        let mut config = Self::defaults(boot_epoch);
        if let Some(raw) = repository_toml {
            config.apply_file(raw, ConfigSource::Repository)?;
        }
        if let Some(raw) = machine_toml {
            config.apply_file(raw, ConfigSource::Machine)?;
        }
        for (key, value) in env {
            config.apply_env(&key, &value)?;
        }
        config.apply_cli(cli);
        config.validate()?;
        config.generation.config = 1;
        Ok(config)
    }

    fn mark_defaults(&mut self) {
        for field in [
            "journal_path",
            "max_frame_size",
            "startup_timeout_ms",
            "shutdown_timeout_ms",
            "quality_floor",
            "require_hive",
        ] {
            self.provenance.insert(
                field.to_owned(),
                ConfigProvenance {
                    source: ConfigSource::CompiledDefault,
                    field: field.to_owned(),
                },
            );
        }
    }

    fn apply_file(&mut self, raw: &str, source: ConfigSource) -> Result<(), ConfigError> {
        let file: FileConfig =
            toml::from_str(raw).map_err(|e| ConfigError::Parse(e.to_string()))?;
        if let Some(value) = file.journal_path {
            self.journal_path = value;
            self.mark_source("journal_path", source);
        }
        if let Some(value) = file.max_frame_size {
            self.max_frame_size = value;
            self.mark_source("max_frame_size", source);
        }
        if let Some(value) = file.startup_timeout_ms {
            self.startup_timeout_ms = value;
            self.mark_source("startup_timeout_ms", source);
        }
        if let Some(value) = file.shutdown_timeout_ms {
            self.shutdown_timeout_ms = value;
            self.mark_source("shutdown_timeout_ms", source);
        }
        if let Some(value) = file.quality_floor {
            self.quality_floor = value;
            self.mark_source("quality_floor", source);
        }
        if let Some(value) = file.require_hive {
            self.require_hive = value;
            self.mark_source("require_hive", source);
        }
        if let Some(value) = file.secret_reference {
            self.secret_reference = Some(SecretReference { name: value });
            self.mark_source("secret_reference", source);
        }
        Ok(())
    }

    fn apply_env(&mut self, key: &str, value: &str) -> Result<(), ConfigError> {
        let source = ConfigSource::Environment;
        match key {
            "CORE_JOURNAL_PATH" => {
                self.journal_path = PathBuf::from(value);
                self.mark_source("journal_path", source);
            }
            "CORE_MAX_FRAME_SIZE" => {
                self.max_frame_size = value.parse().map_err(|_| {
                    ConfigError::Invalid("CORE_MAX_FRAME_SIZE is not an integer".into())
                })?;
                self.mark_source("max_frame_size", source);
            }
            "CORE_STARTUP_TIMEOUT_MS" => {
                self.startup_timeout_ms = value.parse().map_err(|_| {
                    ConfigError::Invalid("CORE_STARTUP_TIMEOUT_MS is not an integer".into())
                })?;
                self.mark_source("startup_timeout_ms", source);
            }
            "CORE_SHUTDOWN_TIMEOUT_MS" => {
                self.shutdown_timeout_ms = value.parse().map_err(|_| {
                    ConfigError::Invalid("CORE_SHUTDOWN_TIMEOUT_MS is not an integer".into())
                })?;
                self.mark_source("shutdown_timeout_ms", source);
            }
            "CORE_QUALITY_FLOOR" => {
                self.quality_floor = value.parse().map_err(|_| {
                    ConfigError::Invalid("CORE_QUALITY_FLOOR is not an integer".into())
                })?;
                self.mark_source("quality_floor", source);
            }
            "CORE_REQUIRE_HIVE" => {
                self.require_hive = parse_bool(value)?;
                self.mark_source("require_hive", source);
            }
            "CORE_SECRET_REFERENCE" => {
                self.secret_reference = Some(SecretReference {
                    name: value.to_owned(),
                });
                self.mark_source("secret_reference", source);
            }
            _ if key.starts_with("CORE_") => {
                return Err(ConfigError::Invalid(format!(
                    "unknown safety configuration key {key}"
                )))
            }
            _ => {}
        }
        Ok(())
    }

    fn apply_cli(&mut self, cli: &ConfigOverrides) {
        let source = ConfigSource::Cli;
        if let Some(value) = &cli.journal_path {
            self.journal_path = value.clone();
            self.mark_source("journal_path", source);
        }
        if let Some(value) = cli.max_frame_size {
            self.max_frame_size = value;
            self.mark_source("max_frame_size", source);
        }
        if let Some(value) = cli.startup_timeout_ms {
            self.startup_timeout_ms = value;
            self.mark_source("startup_timeout_ms", source);
        }
        if let Some(value) = cli.shutdown_timeout_ms {
            self.shutdown_timeout_ms = value;
            self.mark_source("shutdown_timeout_ms", source);
        }
        if let Some(value) = cli.quality_floor {
            self.quality_floor = value;
            self.mark_source("quality_floor", source);
        }
        if let Some(value) = cli.require_hive {
            self.require_hive = value;
            self.mark_source("require_hive", source);
        }
    }

    fn mark_source(&mut self, field: &str, source: ConfigSource) {
        self.provenance.insert(
            field.to_owned(),
            ConfigProvenance {
                source,
                field: field.to_owned(),
            },
        );
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if !(1024..=16 * 1024 * 1024).contains(&self.max_frame_size) {
            return Err(ConfigError::Invalid(
                "max_frame_size must be between 1024 and 16 MiB".into(),
            ));
        }
        if self.startup_timeout_ms == 0 || self.shutdown_timeout_ms == 0 {
            return Err(ConfigError::Invalid("timeouts must be positive".into()));
        }
        if self.quality_floor > 100 {
            return Err(ConfigError::Invalid("quality_floor must be 0..=100".into()));
        }
        if self.journal_path.as_os_str().is_empty()
            || self.journal_path.is_absolute() && self.journal_path.to_string_lossy().contains("..")
        {
            return Err(ConfigError::Invalid("journal_path is unsafe".into()));
        }
        Ok(())
    }

    pub fn reload_from_sources(
        &self,
        repository_toml: Option<&str>,
        machine_toml: Option<&str>,
        env: impl IntoIterator<Item = (String, String)>,
        cli: &ConfigOverrides,
    ) -> Result<Self, ConfigError> {
        let mut reloaded = Self::from_sources(
            repository_toml,
            machine_toml,
            env,
            cli,
            self.generation.boot_epoch,
        )?;
        reloaded.generation.config = self.generation.config.saturating_add(1);
        reloaded.generation.module_graph = self.generation.module_graph;
        reloaded.generation.capability_graph = self.generation.capability_graph;
        reloaded.generation.policy = self.generation.policy;
        Ok(reloaded)
    }

    pub fn redacted_diagnostics(&self) -> serde_json::Value {
        serde_json::json!({
            "journal_path": self.journal_path,
            "max_frame_size": self.max_frame_size,
            "startup_timeout_ms": self.startup_timeout_ms,
            "shutdown_timeout_ms": self.shutdown_timeout_ms,
            "quality_floor": self.quality_floor,
            "require_hive": self.require_hive,
            "secret_reference": self.secret_reference.as_ref().map(|_| "<reference-present>"),
            "generation": self.generation,
            "provenance": self.provenance,
        })
    }
}

fn parse_bool(value: &str) -> Result<bool, ConfigError> {
    match value.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" => Ok(true),
        "0" | "false" | "no" => Ok(false),
        _ => Err(ConfigError::Invalid(
            "boolean configuration value is invalid".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence_is_default_file_env_cli() {
        let cli = ConfigOverrides {
            quality_floor: Some(90),
            ..Default::default()
        };
        let config = CoreConfig::from_sources(
            Some("quality_floor = 60"),
            None,
            [("CORE_QUALITY_FLOOR".into(), "70".into())],
            &cli,
            7,
        )
        .unwrap();
        assert_eq!(config.quality_floor, 90);
        assert_eq!(config.provenance["quality_floor"].source, ConfigSource::Cli);
    }

    #[test]
    fn invalid_safety_key_fails_closed() {
        assert!(CoreConfig::from_sources(
            Some("unknown = 1"),
            None,
            [],
            &ConfigOverrides::default(),
            1
        )
        .is_err());
    }

    #[test]
    fn diagnostics_do_not_contain_secret_value() {
        let config = CoreConfig::from_sources(
            Some("secret_reference = 'vault/core'"),
            None,
            [],
            &ConfigOverrides::default(),
            1,
        )
        .unwrap();
        let text = config.redacted_diagnostics().to_string();
        assert!(!text.contains("vault/core"));
        assert!(text.contains("reference-present"));
    }

    #[test]
    fn safe_reload_advances_only_configuration_generation() {
        let config = CoreConfig::defaults(4);
        let reloaded = config
            .reload_from_sources(
                Some("quality_floor = 80"),
                None,
                [],
                &ConfigOverrides::default(),
            )
            .unwrap();
        assert_eq!(reloaded.quality_floor, 80);
        assert_eq!(reloaded.generation.boot_epoch, 4);
        assert_eq!(reloaded.generation.config, 1);
        assert_eq!(
            reloaded.generation.module_graph,
            config.generation.module_graph
        );
    }
}
