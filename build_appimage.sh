#!/bin/bash
set -e

APP=Portscanner
ARCH=$(uname -m)

# 1. Ordnerstruktur vorbereiten
mkdir -p AppDir/usr/bin

# 2. Binary kopieren
cp target/release/portscanner AppDir/usr/bin/

# 3. .desktop-Datei anlegen
cat > AppDir/${APP}.desktop <<EOF
[Desktop Entry]
Name=Portscanner
Exec=portscanner
Icon=utilities-terminal
Type=Application
Categories=Utility;
Terminal=true
EOF

# 4. AppRun anlegen
cat > AppDir/AppRun <<'EOF'
#!/bin/bash
HERE="$(dirname "$(readlink -f "$0")")"
exec "$HERE/usr/bin/portscanner" "$@"
EOF

chmod +x AppDir/AppRun

# 5. AppImage bauen (aus entpacktem appimagetool)
if [ ! -x squashfs-root/AppRun ]; then
  echo "❌ Du musst appimagetool zuerst extrahieren: ./appimagetool-x86_64.AppImage --appimage-extract"
  exit 1
fi

echo "🚀 Erzeuge AppImage..."
./squashfs-root/AppRun AppDir

# 6. Umbenennen (optional)
mv *.AppImage ${APP}-${ARCH}.AppImage
echo "✅ Fertig: ${APP}-${ARCH}.AppImage"
