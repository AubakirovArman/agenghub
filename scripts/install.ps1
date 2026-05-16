param(
    [string]$Repo = $env:AGENTHUB_REPO,
    [string]$Version = $env:AGENTHUB_VERSION,
    [string]$InstallDir = $env:AGENTHUB_INSTALL_DIR,
    [string]$Artifact = $env:AGENTHUB_ARTIFACT,
    [string]$Checksum = $env:AGENTHUB_CHECKSUM,
    [string]$ChecksumFile = $env:AGENTHUB_CHECKSUM_FILE,
    [string]$SkipChecksum = $env:AGENTHUB_SKIP_CHECKSUM
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($Repo)) {
    $Repo = "AubakirovArman/agenthub"
}
if ([string]::IsNullOrWhiteSpace($Version)) {
    $Version = "latest"
}
if ([string]::IsNullOrWhiteSpace($InstallDir)) {
    $InstallDir = Join-Path $HOME ".agenthub\bin"
}

$arch = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture.ToString()
if ($arch -ne "X64") {
    throw "Unsupported Windows architecture: $arch"
}

$asset = "agenthub-x86_64-pc-windows-msvc.zip"
$tmp = Join-Path ([System.IO.Path]::GetTempPath()) ("agenthub-install-" + [System.Guid]::NewGuid())
New-Item -ItemType Directory -Force -Path $tmp | Out-Null

function Read-AgentHubChecksumFile {
    param([string]$Path)
    $line = Get-Content -Path $Path | Where-Object { -not [string]::IsNullOrWhiteSpace($_) } | Select-Object -First 1
    if ([string]::IsNullOrWhiteSpace($line)) {
        throw "Checksum file is empty: $Path"
    }
    return ($line -split '\s+')[0]
}

function Test-AgentHubChecksum {
    param([string]$Archive)
    if ($SkipChecksum -eq "1") {
        Write-Host "agenthub installer: checksum verification skipped"
        return
    }

    $expected = $Checksum
    if (-not [string]::IsNullOrWhiteSpace($ChecksumFile)) {
        $expected = Read-AgentHubChecksumFile -Path $ChecksumFile
    } elseif ([string]::IsNullOrWhiteSpace($expected)) {
        $adjacentChecksum = "$Archive.sha256"
        if (Test-Path $adjacentChecksum) {
            $expected = Read-AgentHubChecksumFile -Path $adjacentChecksum
        }
    }
    if ([string]::IsNullOrWhiteSpace($expected)) {
        throw "Missing checksum; set AGENTHUB_CHECKSUM, AGENTHUB_CHECKSUM_FILE, or AGENTHUB_SKIP_CHECKSUM=1"
    }

    $actual = (Get-FileHash -Algorithm SHA256 -Path $Archive).Hash.ToLowerInvariant()
    $expected = $expected.ToLowerInvariant()
    if ($actual -ne $expected) {
        throw "Checksum mismatch for ${Archive}: expected $expected, got $actual"
    }
    Write-Host "agenthub installer: checksum verified"
}

try {
    if ([string]::IsNullOrWhiteSpace($Artifact)) {
        $archive = Join-Path $tmp $asset
        if ($Version -eq "latest") {
            $url = "https://github.com/$Repo/releases/latest/download/$asset"
        } else {
            $url = "https://github.com/$Repo/releases/download/$Version/$asset"
        }
        Invoke-WebRequest -Uri $url -OutFile $archive
        if (($SkipChecksum -ne "1") -and [string]::IsNullOrWhiteSpace($Checksum) -and [string]::IsNullOrWhiteSpace($ChecksumFile)) {
            Invoke-WebRequest -Uri "$url.sha256" -OutFile "$archive.sha256"
        }
    } else {
        $archive = $Artifact
    }

    Test-AgentHubChecksum -Archive $archive
    Expand-Archive -Path $archive -DestinationPath $tmp -Force
    $binary = Get-ChildItem -Path $tmp -Filter "agenthub.exe" -Recurse | Select-Object -First 1
    if ($null -eq $binary) {
        throw "Archive does not contain agenthub.exe"
    }

    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    $destination = Join-Path $InstallDir "agenthub.exe"
    Copy-Item -Path $binary.FullName -Destination $destination -Force

    Write-Host "agenthub installed to $destination"
    Write-Host "Add this directory to PATH if needed:"
    Write-Host "  $InstallDir"
} finally {
    Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
}
