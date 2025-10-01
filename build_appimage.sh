#!/bin/bash
set -e

APP=Portscanner
ARCH=$(uname -m)

# Ensure appimagetool exists (download from correct repo)
if [ ! -f appimagetool-$ARCH.AppImage ]; then
  # New repo: AppImage/appimagetool
  wget -O appimagetool-$ARCH.AppImage \
    https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-$ARCH.AppImage
  chmod +x appimagetool-$ARCH.AppImage
fi

# Prepare AppDir
rm -rf AppDir
mkdir -p AppDir/usr/bin

# Copy binary (make sure it's built for correct architecture)
cp target/${ARCH}-unknown-linux-gnu/release/portscanner AppDir/usr/bin/

# Create .desktop
cat > AppDir/${APP}.desktop <<EOF
[Desktop Entry]
Name=Portscanner
Exec=portscanner
Icon=utilities-terminal
Type=Application
Categories=Utility;
Terminal=true
EOF

# Create AppRun launcher
cat > AppDir/AppRun <<'EOF'
#!/bin/bash
HERE="$(dirname "$(readlink -f "$0")")"
exec "$HERE/usr/bin/portscanner" "$@"
EOF
chmod +x AppDir/AppRun

# Build AppImage
echo "🚀 Creating AppImage..."
./appimagetool-$ARCH.AppImage AppDir

# Rename output
mv *.AppImage ${APP}-${ARCH}.AppImage
echo "✅ Done: ${APP}-${ARCH}.AppImage"
