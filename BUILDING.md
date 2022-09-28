# Building

## Build Requirements

To build the application from source, a [Rust toolchain](https://www.rust-lang.org/) is required.

- Use `cargo build` to compile or `cargo run` to compile and run the application in debug mode.
- Use `cargo build --release` to compile or `cargo run --release` to compile and run the application in release mode.

### Windows Installer (optional)

[cargo-wix](https://github.com/volks73/cargo-wix) must be installed to create a Windows installer.

- Run `cargo wix` to build the installer (msi).
- The installer will be created in the `target/wix` folder.

### Mac Application Bundle (optional)

To build a macOS application bundle, additional dependencies must be installed:

- [cargo-bundle](https://github.com/burtonageo/cargo-bundle)
- [Python3](https://python.org) (any recent version should work)

Run `./build-mac-bundle.sh` from the project directory. Make sure the script has executable permissions.

The bundle will be created in the `target/release/bundle/osx` folder.
If the custom app icon does not show up, copy/paste it manually from the icons folder using the finder info dialog.

### Linux AppImage (optional)

To build an AppImage for Linux, additional dependencies must be installed:

- [linuxdeploy](https://github.com/linuxdeploy/linuxdeploy)
- [linuxdeploy-plugin-appimage](https://github.com/linuxdeploy/linuxdeploy-plugin-appimage)

Run `./build-linux-appimage.sh` from the project directory. Make sure the script has executable permissions.
The AppImage will be created in the `./target/release/appimage` directory.
