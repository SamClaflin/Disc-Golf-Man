#!/bin/bash
set -e

cargo build --release

rm -rf dist
mkdir -p dist

if [[ "$OSTYPE" == "darwin"* ]]; then
    APP="dist/Disc-Golf-Man.app"
    mkdir -p "$APP/Contents/MacOS"
    mkdir -p "$APP/Contents/Resources"

    cp target/release/disc_golf_man "$APP/Contents/MacOS/"
    cp -r assets "$APP/Contents/MacOS/assets"

    # Convert icon to icns if sips is available
    if command -v sips &>/dev/null && command -v iconutil &>/dev/null; then
        ICONSET="dist/icon.iconset"
        mkdir -p "$ICONSET"
        sips -z 16 16     assets/tj/tj_closed_left.png --out "$ICONSET/icon_16x16.png"
        sips -z 32 32     assets/tj/tj_closed_left.png --out "$ICONSET/icon_16x16@2x.png"
        sips -z 32 32     assets/tj/tj_closed_left.png --out "$ICONSET/icon_32x32.png"
        sips -z 64 64     assets/tj/tj_closed_left.png --out "$ICONSET/icon_32x32@2x.png"
        sips -z 128 128   assets/tj/tj_closed_left.png --out "$ICONSET/icon_128x128.png"
        sips -z 256 256   assets/tj/tj_closed_left.png --out "$ICONSET/icon_128x128@2x.png"
        sips -z 256 256   assets/tj/tj_closed_left.png --out "$ICONSET/icon_256x256.png"
        iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/AppIcon.icns"
        rm -rf "$ICONSET"
    fi

    cat > "$APP/Contents/Info.plist" << 'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>Disc-Golf-Man</string>
    <key>CFBundleDisplayName</key>
    <string>Disc-Golf-Man</string>
    <key>CFBundleIdentifier</key>
    <string>com.discgolfman.app</string>
    <key>CFBundleExecutable</key>
    <string>disc_golf_man</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
</dict>
</plist>
PLIST

    echo ""
    echo "Build complete: $APP"
else
    cp target/release/disc_golf_man dist/
    cp -r assets dist/assets

    echo ""
    echo "Build complete: dist/disc_golf_man"
fi
