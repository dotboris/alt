#!/bin/bash
set -e

print_help() {
    echo "usage: $0 <target>"
    echo "  target: target to build (osx or linux)"
}

if [ "$#" -lt 1 ]; then
    print_help
    exit 1
fi

target="$1"
cargo_target=

case "$target" in
    osx) cargo_target=x86_64-apple-darwin ;;
    linux) cargo_target=x86_64-unknown-linux-gnu ;;
    *)
        echo "Unknown target: $target"
        print_help
        exit 2
        ;;
esac

cd "$(dirname "$0")/.." || exit 42
mkdir -p dist/release

echo "===> Building $target ($cargo_target)"
cargo build --frozen --release --target "$cargo_target"
echo "===> Copying $target ($cargo_target)"
cp "target/$cargo_target/release/alt" "dist/release/alt_$target"
