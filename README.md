# Töörö Editor

Cross-platform sound editor for the [Fred's Lab Töörö](https://fredslab.net/en/tooro-module.php) hardware synthesizer.

The application is written in Rust and features the [Iced GUI library](https://github.com/hecrj/iced) and the [midir](https://github.com/Boddlnagg/midir) crate for MIDI processing.

## Features

The state of the application can be seen as minimum viable product. Because of its open source nature, user contributions to the code are always welcome.

It currently supports:

- Editing of most preset and multi parameters
- Loading and saving of presets as sysex files

## Usage

Using the editor is mostly self-explanatory, but there are a few things to be noted:

- The Töörö must be connected to the computer via USB. DIN MIDI will not work.
- The Töörö firmware must be V1.5 or higher. Otherwise, not all parameters can be edited.
- The application tries to detect when you change a parameter on the device itself. Unfortunately, this will not work in all cases. Use the **Update from device** button to force a reload of all parameters.
- A manual update must also be requested when you change a preset or change a parameter via MIDI CCs from another application or source.
- All sysex files must use **.syx** as file extension.
- On larger screens, the window width can be increased to improve the resolution of the sliders.

## Known Issues

- The connection state is not always detected correctly when the Töörö is connected or disconnected while the application is running.
- Spacing and alignment of the text labels could be improved.
- Resizing the window height is possible but has no use.
- Using more than one Töörö at a time is not supported.

## Runtime Requirements

The following platforms were tested during development:

- Windows 10
- macOS 10.13 (High Sierra)
- macOS 10.11 (El Capitan) - working, but has some issues

An **OpenGL ES3** compatible graphics adapter is required on Linux. This can be an issue, especially on older notebooks.

## Build Requirements

To build the application from source, a [Rust toolchain](https://www.rust-lang.org/) is required.

- Use `cargo build` to compile or `cargo run` to compile and run the application in debug mode.
- Use `cargo build --release` to compile or `cargo run --release` to compile and run the application in release mode.

### Windows Installer (optional)

[cargo-wix](https://github.com/volks73/cargo-wix) must be installed to create a Windows installer.

- Run `cargo wix` to build the installer (msi).
- The installer will be created in the `target/wix` folder.

### Mac Application Bundle (optional)

[cargo-bundle](https://github.com/burtonageo/cargo-bundle) must be installed to create the macOS app bundle.

- Run `cargo bundle --release` to build the bundle.
- The bundle will be created in the `target/release/bundle/osx` folder.

## Donations

If you like to support my work, you can [buy me a
coffee.](https://www.buymeacoffee.com/sourcebox)

<a href="https://www.buymeacoffee.com/sourcebox" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="41" width="174"></a>