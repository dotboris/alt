#!/bin/sh
set -e -o pipefail

get_latest_version() {
  curl -s --head https://github.com/dotboris/alt/releases/latest \
  | grep '^Location' \
  | grep -Eo 'v[0-9]+(\.[0-9]+)+'
}

get_os() {
  _os="$(uname -s)"

  case "$_os" in
    Linux) printf linux;;
    Darwin) printf osx;;
    *)
      echo "Unkown OS $_os."
      echo "Currently, only linux and osx are supported"
      return 1
      ;;
  esac
}

version="$(get_latest_version)"
os="$(get_os)"

echo "Installing alt $version for $os as /usr/local/bin/alt"
echo "You may be prompted for your password"

file_name="alt_$os"
url="https://github.com/dotboris/alt/releases/download/$version/$file_name"

sudo sh -e -o pipefail -s <<SH
  curl --progress-bar -L "$url" -o /usr/local/bin/alt
  chmod +x /usr/local/bin/alt
SH

echo
echo '  🎉  Alt is installed!  🎉'
echo
echo 'Remember to add $HOME/.local/alt/shims to your PATH'
echo
echo 'BASH:'
echo $'    echo \'export PATH="$HOME/.local/alt/shims:$PATH"\' >> ~/.bashrc'
echo
echo 'ZSH:'
echo $'    echo \'export PATH="$HOME/.local/alt/shims:$PATH"\' >> ~/.zshrc'
echo
echo 'FISH:'
echo $'    echo \'set -x PATH "$HOME/.local/alt/shims" $PATH\' >> ~/.config/fish/config.fish'
echo

