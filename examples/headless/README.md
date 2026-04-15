# Headless mode examples

Example scripts that drive `rust-hf-downloader --headless`.

| Task | POSIX (bash / zsh) | Windows (PowerShell) |
|---|---|---|
| Search | `search-examples.sh` | `search-examples.ps1` |
| Download | `download-examples.sh` | `download-examples.ps1` |
| Timing test | `timing-test.sh` | `timing-test.ps1` |

The `.sh` and `.ps1` variants perform the same work. Pick whichever matches
your shell. The POSIX versions use `jq` / `bc` / `grep`; the PowerShell
versions use `ConvertFrom-Json`, `Measure-Command`, and `Select-String`
instead, so no extra tools are required on Windows.

There is also `ci-example.yml` showing a reference GitHub Actions job that
invokes the CLI.
