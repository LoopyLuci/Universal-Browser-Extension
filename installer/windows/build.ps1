param(
    [string]$StageDir = ".\stage",
    [string]$OutDir = ".\out",
    [string]$Configuration = "release"
)
$ErrorActionPreference = "Stop"

cargo build --release -p dnea-daemon -p dnea-nm-host -p dnea-cli
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

New-Item -ItemType Directory -Force -Path $StageDir | Out-Null
Copy-Item target\release\dnea-daemon.exe $StageDir
Copy-Item target\release\dnea-nm-bridge.exe $StageDir
Copy-Item target\release\dnea.exe $StageDir

New-Item -ItemType Directory -Force -Path "$StageDir\nm" | Out-Null
$bridgePath = (Resolve-Path "$StageDir\dnea-nm-bridge.exe").Path
foreach ($b in @("chrome","edge","firefox","brave")) {
    $origin = if ($b -eq "firefox") { "moz-extension://dnea@dnea.org/" }
              else { "chrome-extension://abcdefghijklmnopabcdefghijklmnop/" }
    $manifest = @{
        name = "org.dnea.runtime"
        description = "DNEA runtime bridge"
        path = $bridgePath
        type = "stdio"
        allowed_origins = @($origin)
        "x-dnea-installed" = @{ version = "1.0.0"; installed_at = (Get-Date -Format "o") }
    }
    $manifest | ConvertTo-Json -Depth 4 | Out-File -Encoding utf8 "$StageDir\nm\org.dnea.runtime.$b.json"
}

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
wix build -arch x64 -d StageDir=$StageDir -o "$OutDir\dnea-1.0.0.msi" dnea.wxs
if ($LASTEXITCODE -ne 0) { throw "wix build failed" }
Write-Host "Built $OutDir\dnea-1.0.0.msi"