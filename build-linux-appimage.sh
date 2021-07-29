#!/bin/bash

cargo build --release

rm -rf target/release/appimage/*

linuxdeploy-x86_64.AppImage \
    --executable ./target/release/tooro-editor \
    --desktop-file ./tools/tooro-editor.desktop \
    --icon-file ./icons/icon_128x128.png \
    --appdir ./target/release/appimage/AppDir \
    --output appimage

echo "Moving appimage to target directory"
mv *.AppImage ./target/release/appimage/
