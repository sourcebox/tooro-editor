# Töörö Editor

Cross-platform sound editor for the [Fred's Lab Töörö](https://fredslab.net/en/tooro-module.php) hardware synthesizer. This application was developed in close cooperation with the manufacturer.

![Screenshot](screenshot.png)

The application is written in Rust and features the [Iced GUI library](https://github.com/hecrj/iced) and the [midir](https://github.com/Boddlnagg/midir) crate for MIDI processing.

## Features

The editor currently supports:

- Editing of most preset and multi parameters
- Loading and saving of presets as sysex files

Being an open source project, user contributions to the code are always welcome.

## Usage

Using the editor is mostly self-explanatory, but there are a few things to be noted:

- The Töörö must be connected to the computer via USB. DIN MIDI will not work.
- When using a DAW at the same time as the editor, make sure it does not loopback sysex messages to the Töörö.
- You can select a MIDI input for playing the Töörö while editing via the **Merge Input** dropdown list on the bottom of the application window.
- Silders can be fine-controlled by holding the *SHIFT* key while dragging.
- To reset a slider value to it's default, use *CTRL*-click or right-click.
- The mouse wheel can also be used to change a slider value.
- The Töörö firmware must be V1.5 or higher. Otherwise, not all parameters can be edited.
- The application tries to detect when you change a parameter on the device itself. Unfortunately, this will not work in all cases. Use the **Update from device** button to force a reload of all parameters.
- A manual update must also be requested when you change a preset or change a parameter via MIDI CCs from another application or source.
- All sysex files must use **.syx** as file extension.
- On larger screens, the window width can be increased to improve the resolution of the sliders.

## Known Issues

- The connection state is not always detected correctly when the Töörö is connected or disconnected while the application is running.
- Resizing the window height is possible but has no use.
- Using more than one Töörö at a time is not supported.

## Runtime Requirements

The following platforms were tested during development:

- Windows 10
- OS X 10.11 (El Capitan)
- macOS 10.13 (High Sierra)
- macOS 11 (Big Sur)
- Linux Mint 20.3

## Building from Source

See [separate document](BUILDING.md) for detailed instructions.

## License

Published under the MIT license. All contributions to this project must be provided under the same license conditions.

Author: Oliver Rockstedt <info@sourcebox.de>

## Donations

If you like to support my work, you can [buy me a
coffee.](https://www.buymeacoffee.com/sourcebox)

<a href="https://www.buymeacoffee.com/sourcebox" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="41" width="174"></a>
