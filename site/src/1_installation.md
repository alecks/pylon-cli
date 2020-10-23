# Installation

`pylon-cli` provides install scripts for most platforms.

- [Windows Installer](https://github.com/alecks/pylon-cli/releases); see the assets in the latest release.
- Shell (macOS, Linux, WSL, other UNIX-like systems):
  ```console
  curl -fsSL https://pylon.alex.lgbt/install.sh | sh
  ```
- Cargo (most platforms, build from source)
  ```console
  git clone https://github.com/alecks/pylon-cli
  cd pylon-cli
  cargo build --release
  mkdir -p ~/.pylon/bin
  cp ./target/release/pylon-cli ~/.pylon/bin/pylon
  ```

> If you're using the shell script or building from source, you'll have to add `~/.pylon/bin` to your `PATH` variable.

Assuming that installation was successful, running `pylon --help` should output a neat help message.
