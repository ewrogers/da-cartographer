#!/bin/sh
set -eu

release_dir=$(CDPATH='' cd "$(dirname "$0")" && pwd)
data_home=${XDG_DATA_HOME:-"$HOME/.local/share"}
app_dir="$data_home/da-cartographer"
app_id=org.dacartographer.Cartographer

mkdir -p "$app_dir" "$data_home/applications" "$data_home/icons/hicolor/512x512/apps"
install -m 755 "$release_dir/cartographer" "$app_dir/cartographer"
if [ -f "$release_dir/LICENSE-Tabler.txt" ]; then
    install -m 644 "$release_dir/LICENSE-Tabler.txt" "$app_dir/"
fi
if [ ! -e "$app_dir/maps.ron" ]; then
    install -m 644 "$release_dir/maps.ron" "$app_dir/maps.ron"
fi
install -m 644 "$release_dir/cartographer.png" "$data_home/icons/hicolor/512x512/apps/$app_id.png"

# The Exec command uses both Desktop Entry string escaping and quoted-argument escaping.
exec_path=$(printf '%s\n' "$app_dir/cartographer" | sed -e 's/["`$\\]/\\&/g' -e 's/\\/\\\\/g' -e 's/%/%%/g')
working_dir=$(printf '%s\n' "$app_dir" | sed 's/\\/\\\\/g')
cat > "$data_home/applications/$app_id.desktop" <<DESKTOP
[Desktop Entry]
Type=Application
Name=DA Cartographer
Comment=Edit Dark Ages maps
Exec="$exec_path"
Path=$working_dir
Icon=$app_id
Terminal=false
StartupWMClass=cartographer
Categories=Graphics;
DESKTOP

printf 'DA Cartographer is ready in your application menu.\n'
