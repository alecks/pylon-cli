#!/bin/sh
# Copy, paste, replace from Deno's install script at https://github.com/denoland/deno_install/blob/master/install.sh
# Copyright 2019 the Deno authors. All rights reserved. MIT license.
# Keep this script simple and easily auditable.

set -e

case $(uname -s) in
Darwin) target="macos" ;;
*) target="linux" ;;
esac

if [ $(uname -m) != "x86_64" ]; then
	echo "Unsupported architecture $(uname -m). Only x64 binaries are available."
	exit
fi

if [ $# -eq 0 ]; then
	pylon_asset_path=$(
		command curl -sSf https://github.com/fjah/pylon-cli/releases |
			command grep -o "/fjah/pylon-cli/releases/download/.*/pylon-cli-${target}\\" |
			command head -n 1
	)
	if [ ! "$pylon_asset_path" ]; then exit 1; fi
	pylon_uri="https://github.com${pylon_asset_path}"
else
	pylon_uri="https://github.com/fjah/pylon-cli/releases/download/${1}/pylon-cli-${target}"
fi

pylon_install="${PYLON_INSTALL:-$HOME/.pylon}"
bin_dir="$pylon_install/bin"
exe="$bin_dir/pylon"

if [ ! -d "$bin_dir" ]; then
	mkdir -p "$bin_dir"
fi

curl --fail --location --progress-bar --output "$exe" "$pylon_uri"
cd "$bin_dir"
chmod +x "$exe"

echo "pylon-cli was installed successfully to $exe"
if command -v pylon >/dev/null; then
	echo "Run 'pylon --help' to get started"
else
	case $SHELL in
	/bin/zsh) shell_profile=".zshrc" ;;
	*) shell_profile=".bash_profile" ;;
	esac
	echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
	echo "  export PYLON_INSTALL=\"$pylon_install\""
	echo "  export PATH=\"\$PYLON_INSTALL/bin:\$PATH\""
	echo "Run '$exe --help' to get started"
fi
