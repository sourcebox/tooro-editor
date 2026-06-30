//! Application messages definitions

use crate::params::Parameter;

#[derive(Debug, Clone)]
pub enum Message {
    /// Native event from the framework.
    EventOccurred(iced::Event),

    /// Modification of a parameter.
    ParameterChange(Parameter, i32),

    /// Change of the selected part via the dropdown menu.
    PartChange(u8),

    /// A new MIDI merge input was selected from the dropdown menu.
    MergeInputChange(String),

    /// Request the update of parameters from the device.
    UpdateFromDevice,

    /// Load sysex after the button was pressed.
    LoadSysexFile,

    /// Save sysex after the button was pressed.
    SavePresetSysexFile,

    /// Regular tick in 1s intervals.
    Tick,

    /// Fast regular ticks for processing more time critical tasks.
    FastTick,

    /// MIDI merge subscription ready, sender is passed as argument.
    MidiMergeSubscriptionReady(iced::futures::channel::mpsc::Sender<Vec<u8>>),

    /// MIDI message from merge input.
    #[allow(clippy::enum_variant_names)]
    MidiMergeInputMessage(Vec<u8>),
}
