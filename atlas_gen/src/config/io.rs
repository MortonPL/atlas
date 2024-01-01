use crate::config::GeneratorConfig;
use std::{fs, path::Path};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Deser(#[from] toml::de::Error),
    #[error("{0}")]
    Serde(#[from] toml::ser::Error),
}

/// Load a generator config from a TOML file.
pub fn load_config(path: impl AsRef<Path>) -> Result<GeneratorConfig> {
    let text = fs::read_to_string(path)?;
    let config = toml::from_str(&text)?;
    Ok(config)
}

/// Save a generator config to a TOML file.
pub fn save_config(config: &GeneratorConfig, path: impl AsRef<Path>) -> Result<()> {
    let text = toml::to_string(config)?;
    fs::write(path, text)?;
    Ok(())
}
