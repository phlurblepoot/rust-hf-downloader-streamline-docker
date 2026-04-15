//! Cross-platform path resolution for config, registry, and the default
//! download directory.
//!
//! Resolution order (highest priority first):
//!
//! 1. Environment variable override
//!    - [`ENV_CONFIG_DIR`] overrides the config directory.
//!    - [`ENV_DATA_DIR`] overrides the data directory (registry + downloads).
//! 2. Portable mode: if `config.toml` sits next to the running executable,
//!    the executable's directory is used for config, and `<exe>/models` is
//!    used for data. This enables running the app from a USB stick or any
//!    user-chosen folder.
//! 3. Platform defaults via the `dirs` crate:
//!    - config: [`dirs::config_dir`] joined with `jreb`
//!    - data:   [`dirs::home_dir`]  joined with `models`
//! 4. Last-resort fallback: [`std::env::temp_dir`] with a subdirectory so
//!    the app never writes directly into the system temp root.

use std::path::PathBuf;

/// Environment variable that overrides the config directory.
pub const ENV_CONFIG_DIR: &str = "RUST_HF_DOWNLOADER_CONFIG_DIR";

/// Environment variable that overrides the data directory (registry + default
/// download location).
pub const ENV_DATA_DIR: &str = "RUST_HF_DOWNLOADER_DATA_DIR";

/// File name of the config; also used as the portable-mode marker.
const CONFIG_FILE: &str = "config.toml";

/// File name of the download registry.
const REGISTRY_FILE: &str = "hf-downloads.toml";

/// Subdirectory used under platform config/data roots.
const APP_DIR: &str = "jreb";

/// Subdirectory (under data root) holding model downloads.
const MODELS_DIR: &str = "models";

/// Returns the directory containing `config.toml`.
pub fn config_dir() -> PathBuf {
    if let Some(p) = env_override(ENV_CONFIG_DIR) {
        return p;
    }
    if let Some(exe_dir) = portable_mode() {
        return exe_dir;
    }
    dirs::config_dir()
        .map(|d| d.join(APP_DIR))
        .unwrap_or_else(|| std::env::temp_dir().join(APP_DIR))
}

/// Returns the full path to `config.toml`.
pub fn config_path() -> PathBuf {
    config_dir().join(CONFIG_FILE)
}

/// Returns the directory that holds the download registry and, by default,
/// the downloaded model files.
pub fn data_dir() -> PathBuf {
    if let Some(p) = env_override(ENV_DATA_DIR) {
        return p;
    }
    if let Some(exe_dir) = portable_mode() {
        return exe_dir.join(MODELS_DIR);
    }
    dirs::home_dir()
        .map(|d| d.join(MODELS_DIR))
        .unwrap_or_else(|| std::env::temp_dir().join(MODELS_DIR))
}

/// The default download directory used when the user has not customised
/// `default_directory` in their config.
pub fn default_download_dir() -> PathBuf {
    data_dir()
}

/// Returns the full path to `hf-downloads.toml`.
pub fn registry_path() -> PathBuf {
    data_dir().join(REGISTRY_FILE)
}

/// Reads an env var and returns `Some(PathBuf)` only when it is set and
/// non-empty.
fn env_override(var: &str) -> Option<PathBuf> {
    std::env::var_os(var)
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}

/// Returns the directory of the current executable if, and only if, a
/// `config.toml` sits next to it (the portable-mode marker).
fn portable_mode() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let exe_dir = exe.parent()?.to_path_buf();
    if exe_dir.join(CONFIG_FILE).exists() {
        Some(exe_dir)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_path_ends_with_config_toml() {
        let path = config_path();
        assert_eq!(path.file_name().and_then(|s| s.to_str()), Some(CONFIG_FILE));
    }

    #[test]
    fn registry_path_ends_with_registry_file() {
        let path = registry_path();
        assert_eq!(
            path.file_name().and_then(|s| s.to_str()),
            Some(REGISTRY_FILE)
        );
    }

    #[test]
    fn env_override_wins_over_defaults() {
        // Use an unlikely-to-collide value; restore afterwards.
        let key = ENV_CONFIG_DIR;
        let saved = std::env::var_os(key);
        std::env::set_var(key, "/tmp/rust-hf-downloader-test-override");
        let dir = config_dir();
        assert_eq!(dir, PathBuf::from("/tmp/rust-hf-downloader-test-override"));
        match saved {
            Some(v) => std::env::set_var(key, v),
            None => std::env::remove_var(key),
        }
    }

    #[test]
    fn empty_env_override_is_ignored() {
        let key = ENV_DATA_DIR;
        let saved = std::env::var_os(key);
        std::env::set_var(key, "");
        // Should fall through to a default — just assert it isn't empty.
        let dir = data_dir();
        assert!(!dir.as_os_str().is_empty());
        match saved {
            Some(v) => std::env::set_var(key, v),
            None => std::env::remove_var(key),
        }
    }
}
