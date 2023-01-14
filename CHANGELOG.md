# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Merge input is remembered between launches.

### Changed

- Migrated `iced` dependency to `0.7`.

## [1.1.0] - 2022-09-28

### Added

- Mouse wheel support for sliders.
- Control-click or right-click to set a slider to its default value.
- Slider fine control by holding shift when dragging.
- Strip symbols from release builds for smaller binaries.
- Disable buttons when device is not connected.

### Changed

- Swapped positions of *Shaper* and *Extra* sections in layout.
- Parameter labels now match the panel printing more precise.
- Lower latency with MIDI merge input.
- Show shorter port names for merge input on Linux.
- Don't show internal port in merge input menu.
- Give merge input menu more space to make longer names fit.
- Show pointer shape mouse cursor when hovering a slider on all platforms.
- Relaxed port name detection for BomeBox compatibility.
- Use 2021 edition of Rust.
- Updated dependencies.

## [1.0.0] - 2021-08-02

First public release.

## [0.1.0] - No date specified

Initial development release for internal use only.
