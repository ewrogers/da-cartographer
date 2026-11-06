#!/bin/sh
set -eu

if [ "$#" -ne 3 ]; then
    printf 'Usage: %s BINARY OUTPUT_DIRECTORY VERSION\n' "$0" >&2
    exit 2
fi

repo=$(CDPATH='' cd "$(dirname "$0")/.." && pwd)
app="$2/DA Cartographer.app"
contents="$app/Contents"
mkdir -p "$contents/MacOS" "$contents/Resources"
install -m 755 "$1" "$contents/MacOS/cartographer"
install -m 644 "$repo/maps.ron" "$contents/Resources/maps.ron"
install -m 644 "$repo/packaging/Info.plist" "$contents/Info.plist"
plutil -replace CFBundleShortVersionString -string "$3" "$contents/Info.plist"
plutil -replace CFBundleVersion -string "$3" "$contents/Info.plist"

temp_dir=$(mktemp -d)
trap 'rm -rf "$temp_dir"' EXIT
iconset="$temp_dir/Cartographer.iconset"
mkdir "$iconset"
for size in 16 32 128 256 512; do
    retina=$((size * 2))
    sips -z "$size" "$size" "$repo/cartographer/app-icon.png" \
        --out "$iconset/icon_${size}x${size}.png" >/dev/null
    sips -z "$retina" "$retina" "$repo/cartographer/app-icon.png" \
        --out "$iconset/icon_${size}x${size}@2x.png" >/dev/null
done
iconutil -c icns "$iconset" -o "$contents/Resources/Cartographer.icns"
codesign --force --sign - "$app"
codesign --verify --strict "$app"
