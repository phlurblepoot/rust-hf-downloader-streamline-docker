# Troubleshooting Guide

This document covers common issues, their causes, and solutions.

## Table of Contents

- [Installation Issues](#installation-issues)
- [Download Problems](#download-problems)
- [Authentication Errors](#authentication-errors)
- [Display Issues](#display-issues)
- [Performance Issues](#performance-issues)
- [File Path Issues](#file-path-issues)
- [Configuration Issues](#configuration-issues)
- [Windows-Specific Issues](#windows-specific-issues)

## Installation Issues

### Rust Version Too Old

**Error**: `error: package requires Rust Edition 2021 but 1.75.0 is below that`

**Solution**: Install a newer Rust version:

```bash
rustup update stable
rustup default stable
```

Or install from source:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Compilation Fails with Missing Dependencies

**Error**: Various linker or dependency errors

**Solution**: Install development dependencies:

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev

# Fedora/RHEL
sudo dnf install gcc openssl-devel
```

### Crates.io Installation Fails

**Error**: `error: could not find `rust-hf-downloader` in registry`

**Solution**: Build from source instead:

```bash
git clone https://github.com/JohannesBertens/rust-hf-downloader.git
cd rust-hf-downloader
cargo install --path .
```

## Download Problems

### Downloads Fail Silently

**Symptom**: Download starts but never completes, no error shown

**Solutions**:

1. Check disk space:
   ```bash
   df -h
   ```

2. Check download directory permissions:
   ```bash
   ls -la ~/models/
   ```

3. Enable rate limiting to prevent network issues:
   - Press `o` to open options
   - Navigate to "Rate Limit" and enable it
   - Set "Max Download Speed" to 25 MB/s

### Cannot Resume Download

**Symptom**: Resume prompts appear but download doesn't continue

**Solutions**:

1. Delete incomplete files and start fresh:
   - On resume popup, press `D` to delete and skip

2. Check registry for corruption:
   ```bash
   cat ~/models/hf-downloads.toml
   ```
   If corrupted, delete it:
   ```bash
   rm ~/models/hf-downloads.toml
   ```

### Multi-part GGUF Files Not Downloading Completely

**Symptom**: Only some parts of a multi-file model download

**Solution**: The application should auto-queue all parts. If not:

1. Press `d` on the quantization group, not individual files
2. Check that all parts have the same base name

### Download Speed Very Slow

**Symptom**: Downloads at <1 MB/s on fast connection

**Solutions**:

1. Increase concurrent threads:
   - Press `o` → Options
   - Navigate to "Concurrent Threads"
   - Increase from default (4) to 8

2. Increase chunk size:
   - Navigate to "Chunk Size (MB)"
   - Increase from default (10) to 50

3. Disable rate limiting if enabled

## Authentication Errors

### 401 Unauthorized for Gated Models

**Error Popup**: Authentication required, shows link to get token

**Solution**:

1. Get token from: https://huggingface.co/settings/tokens
2. Accept model terms on the model's page
3. Press `o` to open Options
4. Navigate to "HuggingFace Token"
5. Press Enter, paste token, Enter again
6. Token saved to `~/.config/jreb/config.toml`

### Token Not Saved

**Symptom**: Must re-enter token on every launch

**Solution**: Check config file permissions:

```bash
# Linux
chmod 600 ~/.config/jreb/config.toml

# macOS
chmod 600 ~/Library/Application\ Support/jreb/config.toml
```

On Windows, per-user file ACLs are applied automatically by the OS when the
file is created in `%APPDATA%`; no `chmod` equivalent is required. Verify the
file isn't marked read-only (right-click &rarr; Properties), and that you
are running the app as the same user who created the config.

### Still Getting 401 After Adding Token

**Solutions**:

1. Verify token is valid:
   ```bash
   curl -H "Authorization: Bearer YOUR_TOKEN" https://huggingface.co/api/user
   ```

2. Ensure you accepted model terms on the model page

3. Check for extra spaces in token field in Options

## Display Issues

### Screen is Blank/Empty

**Symptom**: No content displayed after search or initial load

**Solution**:

1. Try with different terminal themes (dark vs light)
2. Increase terminal font size
3. Check terminal dimensions:
   ```bash
   echo $LINES $COLUMNS
   ```
   Minimum: 24 lines, 80 columns

### Text Contrast Issues

**Symptom**: Text hard to read or invisible

**Solution**:

1. Use terminal's default colors
2. Avoid custom color schemes while running
3. Set terminal background to dark

### UI Elements Misaligned

**Symptom**: Borders, tables, or panels don't line up

**Solution**: Resize terminal to at least 120x40:

```bash
# Check current size
echo $LINES $COLUMNS
# If less than 40x120, resize window
```

## Performance Issues

### High CPU Usage

**Symptom**: Fan spins up, system slows down during downloads

**Solution**: Limit concurrent operations:

1. Reduce concurrent threads (Options → Concurrent Threads)
2. Enable rate limiting with lower value
3. Limit verification concurrency

### Slow Search Results

**Symptom**: Search takes >5 seconds to return

**Solutions**:

1. Check internet connection
2. Use more specific search terms
3. Increase API timeout in config (not exposed in UI)

### Memory Usage Grows

**Symptom**: Application uses more RAM over time

**Solution**: Restart application periodically. Known issue with long-running sessions.

## File Path Issues

### Path Not Found Error

**Symptom**: "Invalid path" or "Path traversal" errors

**Solution**: Use absolute paths:

```bash
# Linux/macOS — instead of:
~/models/
# Use:
/home/username/models/
```

```powershell
# Windows — instead of:
.\models
# Use:
C:\Users\username\models
```

### Permission Denied

**Symptom**: Cannot write to download directory

**Solution**: Fix directory permissions (or pick a location you own):

```bash
# Linux/macOS
mkdir -p ~/models
chmod 755 ~/models
```

```powershell
# Windows
New-Item -ItemType Directory -Path "$env:USERPROFILE\models" -Force
```

On Windows, avoid download directories under UAC-protected paths such as
`C:\Program Files\`, `C:\Windows\`, or `C:\ProgramData\` &mdash; these
require elevation per write and will fail with permission errors. Use
`C:\Users\<you>\models`, a secondary drive (`D:\models`), or the portable
layout described in the README.

### Downloads Go to Wrong Location

**Symptom**: Files saved in unexpected subdirectories

**Solution**: Use simple download path:

1. Press `d` on quantization
2. Edit path to a simple location (e.g. `/home/user/models` on Linux/macOS or
   `C:\Users\user\models` on Windows)
3. Files will be organized as: `<base>/author/model-name/filename`

## Configuration Issues

### Settings Not Persisting

**Symptom**: Options reset on restart

**Solutions**:

1. Check config file exists:
   ```bash
   # Linux
   cat ~/.config/jreb/config.toml
   # macOS
   cat "$HOME/Library/Application Support/jreb/config.toml"
   ```
   ```powershell
   # Windows
   Get-Content "$env:APPDATA\jreb\config.toml"
   ```

2. Fix config file permissions (Unix only; Windows handles ACLs automatically):
   ```bash
   chmod 600 ~/.config/jreb/config.toml
   ```

3. Check disk space isn't full

### Options Screen Doesn't Open

**Symptom**: Pressing `o` does nothing

**Solution**: Ensure you're in the main view (not in a popup):
1. Press `Esc` to close any open popups
2. Press `o` again

### Cannot Find Configuration File

**Default locations**:

| Platform | Path |
|---|---|
| Linux | `~/.config/jreb/config.toml` |
| macOS | `~/Library/Application Support/jreb/config.toml` |
| Windows | `%APPDATA%\jreb\config.toml` |

The location may have been overridden via the `RUST_HF_DOWNLOADER_CONFIG_DIR`
environment variable, or by portable mode (a `config.toml` next to the
executable). See the README for details.

If missing, the application will regenerate defaults on next start.

## Windows-Specific Issues

### TUI renders garbled characters in `cmd.exe`

**Symptom**: Escape sequences (`ESC[2K`, `[0m`, etc.) appear as literal text
instead of styling the output.

**Cause**: Legacy `cmd.exe` builds (pre-Windows 10 1809) do not support ANSI /
VT escape sequences.

**Solution**: Use [Windows Terminal](https://aka.ms/terminal) or PowerShell 7+,
both of which ship with full VT support. On Windows 10 1809 and newer, modern
`cmd.exe` also works.

### Windows Defender SmartScreen warning on first run

**Symptom**: "Windows protected your PC" dialog when launching a downloaded
`rust-hf-downloader.exe`.

**Cause**: The binary is not code-signed.

**Solution**: Click **More info** &rarr; **Run anyway**. As an alternative
that avoids the warning, build from source with
`cargo install rust-hf-downloader`.

### Path too long (`ERROR_PATH_NOT_FOUND`, `os error 3`)

**Symptom**: Downloads fail with a path-not-found error when the full path
would exceed 260 characters (for example, a long model name under a deep
user directory).

**Cause**: The classic Windows `MAX_PATH` limit of 260 characters.

**Solution**: Enable the Win32 long-path feature system-wide. In an elevated
PowerShell session:

```powershell
New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" `
    -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force
```

Reboot, then retry. Alternatively, pick a shorter base directory (e.g.
`C:\m` instead of `C:\Users\longusername\models`) via the Options screen or
`--output`.

### "Config file is being used by another process"

**Symptom**: `save_config` fails with an access-denied error.

**Cause**: Another process (usually a text editor such as Notepad) has the
config file open. Unlike Unix, Windows enforces mandatory file locks.

**Solution**: Close the editor, then retry. The app's `load_config` already
falls back to defaults with a warning if read fails, so only writes are
affected.

### HF_TOKEN environment variable not picked up in a new terminal

**Symptom**: `$env:HF_TOKEN = "..."` works in one PowerShell, but a freshly
opened terminal asks for the token again.

**Cause**: `$env:VAR` assignments in PowerShell are session-local and do not
persist.

**Solution**: Persist the variable for your user:

```powershell
[Environment]::SetEnvironmentVariable("HF_TOKEN", "hf_xxxxxxxxxxxx", "User")
```

Close and reopen the terminal; `rust-hf-downloader --headless search "llama"`
should now pick up the token automatically.

## Still Having Issues?

1. Check the [GitHub Issues](https://github.com/JohannesBertens/rust-hf-downloader/issues)
2. Search existing issues for your problem
3. Open a new issue with:
   - Operating system and version
   - Rust version (`rustc --version`)
   - Terminal emulator used
   - Steps to reproduce
   - Error messages (exact text)
