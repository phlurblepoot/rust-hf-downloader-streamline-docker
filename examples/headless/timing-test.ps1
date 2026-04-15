# Timing test for headless mode download (PowerShell equivalent of timing-test.sh)
# Tests: unsloth/Qwen3-4B-Instruct-2507-GGUF at IQ1_M quantization
$ErrorActionPreference = "Stop"

$ModelId       = "unsloth/Qwen3-4B-Instruct-2507-GGUF"
$Quantization  = "IQ1_M"
$OutputDir     = Join-Path $env:TEMP "rust-hf-downloader-timing-test"
$DryRunOutput  = Join-Path $env:TEMP "rust-hf-downloader-dry-run-output.txt"
$DownloadLog   = Join-Path $env:TEMP "rust-hf-downloader-download-output.txt"

Write-Host "=== Rust HF Downloader Timing Test ==="
Write-Host "Model: $ModelId"
Write-Host "Quantization: $Quantization"
Write-Host "Output directory: $OutputDir"
Write-Host ""

if (Test-Path $OutputDir) {
    Write-Host "Removing existing download at: $OutputDir"
    Remove-Item -Recurse -Force $OutputDir
}
New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

# Time the dry run to measure API response time.
Write-Host "1. Timing dry run (measuring API response time)..."
$dryRun = Measure-Command {
    cargo run --release -- --headless --dry-run download `
        $ModelId `
        --quantization $Quantization `
        --output $OutputDir *> $DryRunOutput
}
$DryRunTime = [math]::Round($dryRun.TotalSeconds, 3)
Write-Host "Dry run completed in: ${DryRunTime}s"
Write-Host ""

# Extract file count and total size from dry run output.
$filesLine = Select-String -Path $DryRunOutput -Pattern "Files to download:" | Select-Object -First 1
$FileCount = if ($filesLine) { ($filesLine.Line -replace '[^0-9]', '') } else { "?" }

$sizeLine = Select-String -Path $DryRunOutput -Pattern "Total size:" | Select-Object -First 1
$TotalSizeRaw = if ($sizeLine) {
    ($sizeLine.Line -split ':', 2)[1].Trim()
} else {
    "? ?"
}
Write-Host "Files to download: $FileCount"
Write-Host "Total size: $TotalSizeRaw"
Write-Host ""

# Convert total size to MB for later speed calculation.
$TotalSizeMb = 0.0
if ($TotalSizeRaw -match '([0-9]+(?:\.[0-9]+)?)\s*(GB|MB|KB|B)') {
    $value = [double]$matches[1]
    switch ($matches[2]) {
        'GB' { $TotalSizeMb = $value * 1024 }
        'MB' { $TotalSizeMb = $value }
        'KB' { $TotalSizeMb = $value / 1024 }
        'B'  { $TotalSizeMb = $value / (1024 * 1024) }
    }
}

# Time the actual download.
Write-Host "2. Timing actual download..."
$download = Measure-Command {
    cargo run --release -- --headless download `
        $ModelId `
        --quantization $Quantization `
        --output $OutputDir *> $DownloadLog
}
$DownloadTime = [math]::Round($download.TotalSeconds, 3)
$DownloadSpeed = if ($DownloadTime -gt 0) {
    [math]::Round($TotalSizeMb / $DownloadTime, 2)
} else {
    0
}
Write-Host "Download completed in: ${DownloadTime}s"
Write-Host "Average speed: ${DownloadSpeed} MB/s"
Write-Host ""

Write-Host "=== Timing Test Summary ==="
Write-Host "Model: $ModelId"
Write-Host "Quantization: $Quantization"
Write-Host "Files: $FileCount"
Write-Host ("Total size: {0} ({1} MB)" -f $TotalSizeRaw, [math]::Round($TotalSizeMb, 2))
Write-Host "Dry run time: ${DryRunTime}s"
Write-Host "Download time: ${DownloadTime}s"
Write-Host "Average speed: ${DownloadSpeed} MB/s"
Write-Host ""

Remove-Item -Force -ErrorAction SilentlyContinue $DryRunOutput, $DownloadLog

Write-Host "Timing test completed!"
