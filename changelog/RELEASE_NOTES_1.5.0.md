# Release Notes - Version 1.5.0

**Release Date**: 2026-04-15

## Highlights

- **Windows support**: the app now builds, installs, and runs correctly on
  Windows 10 1809+ / Windows 11 with the MSVC toolchain.
- **Portable mode** and two new env vars for running the app from a
  USB stick or any user-chosen folder.
- **Cross-platform CI**: new GitHub Actions workflow tests every PR on
  Linux, macOS, and Windows, and uploads per-OS release binaries on tags.
- **Path sanitizer hardening** against Windows-reserved filenames and
  illegal characters (defensive on all platforms).

## Feature: Cross-platform path resolution

### Problem

Three sites in the codebase hardcoded Unix-only `$HOME` lookups with a
`/tmp` fallback (`src/config.rs`, `src/registry.rs`, `src/models.rs`). On
Windows, `$HOME` is usually unset, so config files and downloads landed in
`/tmp/.config/jreb/...` — a non-existent path that the OS couldn't create.

### Solution

A new `src/paths.rs` module centralises path resolution using the `dirs`
crate for platform defaults. Resolution order (highest priority first):

1. Environment variable override
   - `RUST_HF_DOWNLOADER_CONFIG_DIR` → config directory
   - `RUST_HF_DOWNLOADER_DATA_DIR` → data directory (registry + downloads)
2. Portable mode: if `config.toml` exists next to the running executable,
   the exe's directory is used for config and `<exe>/models` for data
3. Platform defaults:
   - Linux: `~/.config/jreb/config.toml` + `~/models/`
   - macOS: `~/Library/Application Support/jreb/config.toml` + `~/models/`
   - Windows: `%APPDATA%\jreb\config.toml` + `%USERPROFILE%\models\`
4. Last-resort fallback: `std::env::temp_dir()` with a per-app subdir

### Windows installation

```powershell
# From source (until a pre-built binary is on the Releases page)
cargo install rust-hf-downloader

# From the Releases page
# Download rust-hf-downloader-windows-x86_64.zip, extract, place the
# .exe on PATH.
```

Prerequisites: Visual Studio Build Tools 2019+ (Desktop development with
C++ workload) so `link.exe` is on PATH. A modern terminal is recommended
for the TUI (Windows Terminal or PowerShell 7+).

See README's `### Windows` section for the full walkthrough including
`HF_TOKEN` persistence and portable-mode setup.

## Feature: Path sanitizer hardening

### Problem

`sanitize_path_component` in `src/download.rs` already rejected path
traversal (`..`, `/`, `\`, `\0`) and trailing dots, but didn't catch:

- ASCII control characters (0x00-0x1F, 0x7F)
- Windows-illegal characters (`< > : " | ? *`)
- Windows reserved device names (`CON`, `PRN`, `AUX`, `NUL`, `COM1-9`,
  `LPT1-9`) — case-insensitive, with or without a file extension

### Solution

Extended the sanitizer to reject all of the above on every platform, so
downloads that would have produced unusable or reserved filenames now
fail fast with a clear error instead of silently creating broken files.
Similar-looking-but-safe names like `console.log` or `concert.txt` are
still accepted.

Six new unit tests cover the new rejections (see
`src/download.rs::tests`).

## Feature: Cross-platform CI + release automation

A new `.github/workflows/ci.yml` runs on every push to `main`, every PR,
and every `v*` tag:

- `test` matrix on `ubuntu-22.04`, `macos-latest`, `windows-latest`:
  `cargo fmt --check`, `cargo clippy --all-targets -D warnings`,
  `cargo build --release`, `cargo test`
- `release` matrix (tag-gated, depends on `test` passing): per-OS release
  build, packages as `.tar.gz` (Linux/macOS) or `.zip` (Windows),
  attaches artifacts to the auto-created GitHub Release via
  `softprops/action-gh-release@v2`

This release is the first to carry auto-uploaded pre-built binaries.

## Additions for headless users

- PowerShell ports of the existing shell example scripts:
  `examples/headless/search-examples.ps1`,
  `examples/headless/download-examples.ps1`,
  `examples/headless/timing-test.ps1`.
- `examples/headless/README.md` documenting the `.sh` / `.ps1` pairing.

## Changes

**Files added:**
- `src/paths.rs` — cross-platform path resolution module
- `.github/workflows/ci.yml` — CI + release automation
- `examples/headless/*.ps1` — PowerShell examples
- `examples/headless/README.md`
- `changelog/RELEASE_NOTES_1.5.0.md` (this file)

**Files modified:**
- `Cargo.toml` — `dirs = "5.0"` dependency; version bumped to 1.5.0
- `Cargo.lock` — regenerated
- `src/cli.rs` — clap `version` attribute bumped to 1.5.0
- `src/config.rs` — delegates to `paths::config_path()`; test rewritten
  to be platform-agnostic
- `src/registry.rs` — delegates to `paths::registry_path()`
- `src/models.rs` — `AppOptions::default` uses `paths::default_download_dir()`
- `src/download.rs` — `sanitize_path_component` hardening + 6 new tests
- `src/main.rs` — registers `mod paths`
- `README.md` — Windows installation, Portable mode, per-platform path
  tables, Authentication Windows examples
- `TROUBLESHOOTING.md` — new "Windows-Specific Issues" section covering
  long paths, SmartScreen, VT-compatible terminals, `HF_TOKEN` persistence,
  UAC-protected dirs

## Compatibility

- **Linux**: fully backward compatible. `dirs::config_dir()` returns
  `~/.config` on Linux, so existing users find their `~/.config/jreb/`
  unchanged.
- **macOS**: **breaking** — the config path moves from
  `~/.config/jreb/config.toml` to
  `~/Library/Application Support/jreb/config.toml`. Existing users need
  to either copy their old config to the new location, or set
  `RUST_HF_DOWNLOADER_CONFIG_DIR="$HOME/.config/jreb"` to keep the old
  layout.
- **Windows**: new platform — no prior state to preserve.
- **TUI mode**, **CLI mode**, **downloads**, **verification**,
  **rate-limiting**: no behavioural changes.

## Upgrade recommendation

- **Recommended** for any Windows user (this is the first release that
  actually works on Windows).
- **Recommended** for macOS users who want to benefit from a cleaner path
  layout and the new portable-mode / env-var overrides, with the caveat
  above.
- **Optional** for Linux users — no user-visible changes, but the new CI
  and pre-built release binary may be convenient.

## Migration

- **From source (any platform)**:
  ```
  cargo install rust-hf-downloader
  ```
- **Windows pre-built binary**: download
  `rust-hf-downloader-windows-x86_64.zip` from the
  [v1.5.0 Release page](https://github.com/phlurblepoot/rust-hf-downloader-streamline-docker/releases/tag/v1.5.0),
  extract, put the `.exe` on PATH.
- **macOS config path migration** (only if you had a prior install):
  ```
  mkdir -p "$HOME/Library/Application Support/jreb"
  mv "$HOME/.config/jreb/config.toml" "$HOME/Library/Application Support/jreb/"
  ```
  Or keep the old location via
  `export RUST_HF_DOWNLOADER_CONFIG_DIR="$HOME/.config/jreb"` in your
  shell profile.
