# Installation

`pylon-cli` provides install scripts for most platforms.

- Shell (macOS, Linux, WSL, other UNIX-like systems):
  ```console
  curl -fsSL https://pylon.alex.lgbt/i/install.sh | sh
  ```
- Powershell (Windows)
  ```ps
  iwr https://pylon.alex.lgbt/i/install.ps1 -useb | iex
  ```
- Cargo (most platforms, build from source)
  ```console
  git clone https://github.com/fjah/pylon-cli
  cd pylon-cli
  cargo build --release
  cp ./target/release/pylon-cli /usr/bin/pylon
  ```
