param(
    [string]$SeedDir = "seeds/candidate-latin-english-100",
    [string]$FixtureDir = "fixtures/generated/candidate-latin-english-100",
    [string]$ReportDir = "reports/candidate-latin-english-100"
)

$ErrorActionPreference = "Stop"

& powershell -ExecutionPolicy Bypass -File tools/generate_candidate_seed_batch.ps1 -OutputDir $SeedDir
if ($LASTEXITCODE -ne 0) {
    throw "seed batch generation failed"
}

& cargo build -p lexis-cli
if ($LASTEXITCODE -ne 0) {
    throw "lexis-cli build failed"
}

$fixturePath = [System.IO.Path]::GetFullPath($FixtureDir)
$repoPath = [System.IO.Path]::GetFullPath((Get-Location).Path)
if (-not $fixturePath.StartsWith($repoPath)) {
    throw "refusing to clean fixture dir outside repo: $fixturePath"
}
if (Test-Path -LiteralPath $FixtureDir) {
    Remove-Item -LiteralPath $FixtureDir -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $FixtureDir | Out-Null
New-Item -ItemType Directory -Force -Path $ReportDir | Out-Null

$generated = 0
Get-ChildItem -LiteralPath $SeedDir -Filter *.yaml |
    Sort-Object Name |
    ForEach-Object {
        $name = [IO.Path]::GetFileNameWithoutExtension($_.Name)
        $outDir = Join-Path $FixtureDir $name
        New-Item -ItemType Directory -Force -Path $outDir | Out-Null
        & target/debug/lexis.exe slice generate $_.FullName (Join-Path $outDir "fixture.yaml") | Out-Null
        if ($LASTEXITCODE -ne 0) {
            throw "slice generation failed for $($_.FullName)"
        }
        $generated++
    }

& target/debug/lexis.exe batch validate $FixtureDir |
    Tee-Object -FilePath (Join-Path $ReportDir "batch-validate.txt")
if ($LASTEXITCODE -ne 0) {
    throw "batch validation command failed"
}

& target/debug/lexis.exe batch summary $FixtureDir |
    Tee-Object -FilePath (Join-Path $ReportDir "batch-summary.txt")
if ($LASTEXITCODE -ne 0) {
    throw "batch summary command failed"
}

Write-Output "candidate_pipeline_complete: $FixtureDir"
Write-Output "generated_fixtures: $generated"
Write-Output "reports: $ReportDir"
