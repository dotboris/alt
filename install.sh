#!/bin/sh
set -e -o pipefail

get_latest_version() {
  curl -s --head https://github.com/dotboris/alt/releases/latest \
  | grep '^Location' \
  | grep -Eo 'v[0-9]+(\.[0-9]+)+'
}

get_os() {
  local os;
  os="$(uname -s)"

  case "$os" in
    Linux) echo -n linux;;
    Darwin) echo -n osx;;
    *)
      echo "Unkown OS $os."
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
