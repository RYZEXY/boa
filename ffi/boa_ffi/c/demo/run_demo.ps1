$ErrorActionPreference = 'Stop'
$repoRoot = 'C:\Users\sunfr\projects\boa'
$manifestPath = Join-Path $repoRoot 'ffi/boa_ffi/Cargo.toml'
$rustLib = Join-Path $repoRoot 'target/debug/boa_ffi.dll.lib'
$bin = Join-Path $PSScriptRoot 'demo.exe'

$env:Path += ';C:\Users\sunfr\.cargo\bin;C:\MinGW\bin'
Push-Location $repoRoot
cargo build --lib --manifest-path $manifestPath
if (-not (Test-Path $rustLib)) {
    throw "Rust library not found at $rustLib"
}

gcc -std=c11 -I (Join-Path $PSScriptRoot '..\generated') (Join-Path $PSScriptRoot 'main.c') $rustLib -o $bin
Pop-Location
& $bin
