//! Application messages definitions

use crate::params::{MultiParameter, SoundParameter};

#[derive(Debug, Clone)]
pub enum Message {
    /// Native event from the framework
    EventOccurred(iced_native::Event),

    /// Modification of a sound (preset) parameter
    SoundParameterChange(SoundParameter, i32),

    /// Modification of a a multi parameter
    MultiParameterChange(MultiParameter, i32),

    /// Change of the selected part via the dropdown menu
    PartChange(u8),

    /// A new MIDI merge input was selected from the dropdown menu
    MergeInputChange(String),

    /// Request the update of parameters from the device
    UpdateFromDevice,

    /// Load sysex after the button was pressed
    LoadSysexFile,

    /// Save sysex after the button was pressed
    SavePresetSysexFile,

    /// Regular tick in 1s intervals
    Tick,

    /// Fast regular ticks for processing more time critical tasks
    FastTick,
}
