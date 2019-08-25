#!/bin/bash
set -e -o pipefail

get_latest_version() {
  curl -s --head https://github.com/dotboris/alt/releases/latest \
  | grep '^Location' \
  | grep -Eo 'v[0-9]+(\.[0-9]+)+'
}

get_std_arch() {
  local _arch
  _arch="$(uname -m)"

  case "$_arch" in
    i386|i686) printf 'i686';;
    *) printf '%s' "$_arch";;
  esac
}

get_target() {
  local _arch
  _arch="$(get_std_arch)"
  local _os
  _os="$(uname -s | tr '[:upper:]' '[:lower:]')"

  case "$_os-$_arch" in
    linux-x86_64) printf 'x86_64-unknown-linux-musl';;
    darwin-x86_64) printf 'x86_64-apple-darwin';;
    *)
      echo "Unsupported OS/Arch $_os/$_arch. Try building by hand."
      return 1
      ;;
  esac
}

version="$(get_latest_version)"
target="$(get_target)"

echo "Installing alt $version for $target as /usr/local/bin/alt"
echo "You may be prompted for your password"

file_name="alt_${version}_${target}.gz"
url="https://github.com/dotboris/alt/releases/download/$version/$file_name"

sudo bash -e -o pipefail -s <<SH
  curl --progress-bar -L "$url" -o /usr/local/bin/alt.gz
  rm -f /usr/local/bin/alt
  gzip -d /usr/local/bin/alt.gz
  rm -f /usr/local/bin/alt.gz
  chmod +x /usr/local/bin/alt
SH

echo
echo '  🎉  Alt is installed!  🎉'
echo
echo 'Remember to add $HOME/.local/alt/shims to your PATH'
echo
echo 'BASH:'
echo $'    echo \'export PATH="$HOME/.local/alt/shims:$PATH"\' >> ~/.bashrc'
echo $'    export PATH="$HOME/.local/alt/shims:$PATH"'
echo
echo 'ZSH:'
echo $'    echo \'export PATH="$HOME/.local/alt/shims:$PATH"\' >> ~/.zshrc'
echo $'    export PATH="$HOME/.local/alt/shims:$PATH"'
echo
echo 'FISH:'
echo $'    echo \'set -x PATH "$HOME/.local/alt/shims" $PATH\' >> ~/.config/fish/config.fish'
echo $'    set -x PATH "$HOME/.local/alt/shims" $PATH'
echo

