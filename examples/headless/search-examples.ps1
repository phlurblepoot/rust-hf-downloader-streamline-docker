# Search examples for headless mode (PowerShell equivalent of search-examples.sh)
$ErrorActionPreference = "Stop"

Write-Host "=== Search Examples ==="

Write-Host "`n1. Basic search:"
rust-hf-downloader --headless search "llama"

Write-Host "`n2. Popular models:"
rust-hf-downloader --headless search "gpt" `
  --min-downloads 10000 `
  --min-likes 100

Write-Host "`n3. JSON output:"
# ConvertFrom-Json replaces jq; the pipeline below prints id + downloads per result.
$json = rust-hf-downloader --headless --json search "stable diffusion" | Out-String
$data = $json | ConvertFrom-Json
$data.results | ForEach-Object { [PSCustomObject]@{ id = $_.id; downloads = $_.downloads } } | Format-Table

Write-Host "`n4. Recently updated:"
rust-hf-downloader --headless search "llama" --sort modified
