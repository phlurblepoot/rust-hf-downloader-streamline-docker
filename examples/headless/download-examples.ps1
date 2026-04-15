# Download examples for headless mode (PowerShell equivalent of download-examples.sh)
$ErrorActionPreference = "Stop"

$ModelId   = "TheBloke/TinyLlama-1.1B-Chat-v0.3-GGUF"
$OutputDir = Join-Path $env:TEMP "rust-hf-downloader-models"

Write-Host "=== Download Examples ==="

Write-Host "`n1. Dry run (see what would be downloaded):"
rust-hf-downloader --headless --dry-run download `
  $ModelId `
  --quantization "Q4_K_M"

Write-Host "`n2. Download specific quantization:"
# Uncomment to actually download:
# rust-hf-downloader --headless download `
#   $ModelId `
#   --quantization "Q4_K_M" `
#   --output $OutputDir

Write-Host "`n3. List available files:"
rust-hf-downloader --headless list $ModelId
