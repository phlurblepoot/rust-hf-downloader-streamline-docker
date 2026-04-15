use crate::models::AppOptions;
use std::fs;
use std::path::PathBuf;

/// Get the path to the configuration file
pub fn get_config_path() -> PathBuf {
    crate::paths::config_path()
}

/// Ensure the config directory exists
fn ensure_config_dir() -> Result<(), std::io::Error> {
    let config_path = get_config_path();
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// Load configuration from disk, or return defaults if not found
pub fn load_config() -> AppOptions {
    let path = get_config_path();

    if !path.exists() {
        return AppOptions::default();
    }

    match fs::read_to_string(&path) {
        Ok(contents) => match toml::from_str::<AppOptions>(&contents) {
            Ok(options) => options,
            Err(e) => {
                eprintln!(
                    "Warning: Failed to parse config file: {}. Using defaults.",
                    e
                );
                AppOptions::default()
            }
        },
        Err(e) => {
            eprintln!(
                "Warning: Failed to read config file: {}. Using defaults.",
                e
            );
            AppOptions::default()
        }
    }
}

/// Save configuration to disk
pub fn save_config(options: &AppOptions) -> Result<(), Box<dyn std::error::Error>> {
    ensure_config_dir()?;

    let toml_string = toml::to_string_pretty(options)?;
    fs::write(get_config_path(), toml_string)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_path() {
        let path = get_config_path();
        // The file name is constant on every platform.
        assert_eq!(
            path.file_name().and_then(|s| s.to_str()),
            Some("config.toml"),
            "unexpected config path: {:?}",
            path
        );
        // The path must have a parent directory; we never want the config to
        // be written at the filesystem root.
        assert!(
            path.parent().is_some_and(|p| !p.as_os_str().is_empty()),
            "config path has no meaningful parent: {:?}",
            path
        );
    }

    #[test]
    fn test_load_nonexistent_config() {
        // Should return defaults without panicking
        let options = load_config();
        assert_eq!(options.concurrent_threads, 8);
    }
}
