Write-Output "Installing rust components..."
rustup default stable
rustup component add rustfmt clippy
Write-Output "Bootstrapped developer toolchain."
