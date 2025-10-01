#!/bin/bash
set -e

APP=Portscanner
ARCH=$(uname -m)

# Ensure appimagetool exists
if [ ! -f appimagetool.AppImage ]; then
  wget -O appimagetool.AppImage https://github.com/AppImage/AppImageKit/releases/latest/download/appimagetool-x86_64.AppImage
  chmod +x appimagetool.AppImage
fi

# Prepare AppDir
rm -rf AppDir
mkdir -p AppDir/usr/bin

# Copy binary
cp target/x86_64-unknown-linux-gnu/release/portscanner AppDir/usr/bin/

# .desktop file
cat > AppDir/${APP}.desktop <<EOF
[Desktop Entry]
Name=Portscanner
Exec=portscanner
Icon=utilities-terminal
Type=Application
Categories=Utility;
Terminal=true
EOF

# AppRun file
cat > AppDir/AppRun <<'EOF'
#!/bin/bash
HERE="$(dirname "$(readlink -f "$0")")"
exec "$HERE/usr/bin/portscanner" "$@"
EOF
chmod +x AppDir/AppRun

# Build AppImage
echo "🚀 Creating AppImage..."
./appimagetool.AppImage AppDir

# Rename
mv *.AppImage ${APP}-${ARCH}.AppImage
echo "✅ Done: ${APP}-${ARCH}.AppImage"
