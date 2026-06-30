//! Definitions and methods all parameters.

use std::collections::HashMap;
use std::ops::RangeInclusive;

/// Wrapper for all parameters.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Parameter {
    /// Sound parameter.
    Sound(SoundParameter),

    /// Multi parameter.
    Multi(MultiParameter),
}

impl Parameter {
    /// Returns the value range of the parameter.
    pub fn get_range(&self) -> RangeInclusive<i32> {
        match self {
            Self::Sound(param) => param.get_range(),
            Self::Multi(param) => param.get_range(),
        }
    }

    pub fn get_default(&self) -> i32 {
        match self {
            Self::Sound(param) => param.get_default(),
            Self::Multi(param) => param.get_default(),
        }
    }
}

/// Preset parameters.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SoundParameter {
    // Osc 1
    Osc1Wave,
    Osc1Coarse,
    Osc1FMAmount,
    Osc1Level,
    Osc1Table,
    Osc1Fine,
    Osc1FMRate,
    Osc1Sync,

    // Osc 2
    Osc2Wave,
    Osc2FMAmount,
    Osc2Level,
    Osc2Coarse,
    Osc2Table,
    Osc2Fine,
    Osc2FMRate,
    Osc2Sync,

    // Extra
    ExtraNoise,
    ExtraRingMod,

    // Filter
    FilterCutoff,
    FilterResonance,
    FilterEnvFAmount,
    FilterTrack,
    FilterAfter,
    FilterLFO1Amount,

    // Shaper
    ShaperCutoff,
    ShaperResonance,
    ShaperEnvAAmount,
    ShaperTrack,
    ShaperMode,
    ShaperLFO2Amount,

    // Env F
    EnvFAttack,
    EnvFDecay,
    EnvFSustain,
    EnvFRelease,
    EnvFVelo,
    EnvFHold,
    EnvFAfter,
    EnvFTrigger,

    // Env A
    EnvAAttack,
    EnvADecay,
    EnvASustain,
    EnvARelease,
    EnvAVelo,
    EnvAHold,
    EnvAAfter,
    EnvATrigger,

    // LFO 1
    LFO1Shape,
    LFO1Speed,
    LFO1Rise,
    LFO1Phase,

    // LFO 2
    LFO2Shape,
    LFO2Speed,
    LFO2Rise,
    LFO2Phase,

    // Arpeggiator
    ArpMode,
    ArpGrid,
    ArpTempo,
    ArpHold,

    // Amplifier
    AmpLevel,
    AmpPan,

    // Modulations
    ModEnvFAmount,
    ModEnvFTarget,
    ModEnvAAmount,
    ModEnvATarget,
    ModLFO1Amount,
    ModLFO1Target,
    ModLFO2Amount,
    ModLFO2Target,
    ModModwheelAmount,
    ModModwheelTarget,
    ModPitchAmount,
    ModPitchTarget,
    ModVelocityAmount,
    ModVelocityTarget,
    ModAftertouchAmount,
    ModAftertouchTarget,

    // Misc
    BendRange,
    Tune,
    PolyMode,
}

impl SoundParameter {
    /// Returns the value range of the parameter.
    pub fn get_range(&self) -> RangeInclusive<i32> {
        match self {
            // Default for bipolar
            SoundParameter::Osc1FMRate
            | SoundParameter::Osc2FMRate
            | SoundParameter::ShaperEnvAAmount
            | SoundParameter::ShaperLFO2Amount
            | SoundParameter::FilterEnvFAmount
            | SoundParameter::FilterLFO1Amount
            | SoundParameter::FilterAfter
            | SoundParameter::AmpPan
            | SoundParameter::ModEnvFAmount
            | SoundParameter::ModEnvAAmount
            | SoundParameter::ModLFO1Amount
            | SoundParameter::ModLFO2Amount
            | SoundParameter::ModModwheelAmount
            | SoundParameter::ModPitchAmount
            | SoundParameter::ModVelocityAmount
            | SoundParameter::ModAftertouchAmount => RangeInclusive::new(-128, 128),

            // Special ranges
            SoundParameter::Osc1Coarse | SoundParameter::Osc2Coarse => RangeInclusive::new(-36, 36),
            SoundParameter::Osc1Fine | SoundParameter::Osc2Fine => RangeInclusive::new(-99, 99),
            SoundParameter::ShaperTrack | SoundParameter::FilterTrack => {
                RangeInclusive::new(-20, 20)
            }
            SoundParameter::ArpTempo => RangeInclusive::new(1, 199),
            SoundParameter::BendRange => RangeInclusive::new(0, 127),
            SoundParameter::Tune => RangeInclusive::new(-99, 99),

            // Lists
            SoundParameter::Osc1Table | SoundParameter::Osc2Table => RangeInclusive::new(0, 10),
            SoundParameter::ShaperMode => RangeInclusive::new(0, 2),
            SoundParameter::EnvATrigger | SoundParameter::EnvFTrigger => RangeInclusive::new(0, 2),
            SoundParameter::LFO1Shape | SoundParameter::LFO2Shape => RangeInclusive::new(0, 7),
            SoundParameter::LFO1Phase | SoundParameter::LFO2Phase => RangeInclusive::new(0, 5),
            SoundParameter::ArpMode => RangeInclusive::new(0, 7),
            SoundParameter::ArpGrid => RangeInclusive::new(0, 6),
            SoundParameter::ModEnvFTarget
            | SoundParameter::ModEnvATarget
            | SoundParameter::ModLFO1Target
            | SoundParameter::ModLFO2Target
            | SoundParameter::ModModwheelTarget
            | SoundParameter::ModPitchTarget
            | SoundParameter::ModVelocityTarget
            | SoundParameter::ModAftertouchTarget => RangeInclusive::new(0, 21),

            // Boolean
            SoundParameter::ArpHold | SoundParameter::PolyMode => RangeInclusive::new(0, 1),

            // Default
            _ => RangeInclusive::new(0, 255),
        }
    }

    /// Returns the default value for the parameter.
    pub fn get_default(&self) -> i32 {
        0
    }
}

/// Multi parameters.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MultiParameter {
    // Preset IDs
    PresetPart1,
    PresetPart2,
    PresetPart3,
    PresetPart4,

    // MIDI channels
    ChannelPart1,
    ChannelPart2,
    ChannelPart3,
    ChannelPart4,

    // Volumes
    VolumePart1,
    VolumePart2,
    VolumePart3,
    VolumePart4,

    // Balances
    BalancePart1,
    BalancePart2,
    BalancePart3,
    BalancePart4,

    // FX
    FXLength,
    FXFeedback,
    FXMix,
    FXMode,
    FXSpeed,
    FXDepth,
}

impl MultiParameter {
    /// Returns the value range of the parameter.
    pub fn get_range(&self) -> RangeInclusive<i32> {
        match self {
            // Preset IDs
            MultiParameter::PresetPart1
            | MultiParameter::PresetPart2
            | MultiParameter::PresetPart3
            | MultiParameter::PresetPart4 => RangeInclusive::new(0, 99),

            // MIDI channels
            MultiParameter::ChannelPart1
            | MultiParameter::ChannelPart2
            | MultiParameter::ChannelPart3
            | MultiParameter::ChannelPart4 => RangeInclusive::new(0, 15),

            // Volumes
            MultiParameter::VolumePart1
            | MultiParameter::VolumePart2
            | MultiParameter::VolumePart3
            | MultiParameter::VolumePart4 => RangeInclusive::new(0, 255),

            // Balances
            MultiParameter::BalancePart1
            | MultiParameter::BalancePart2
            | MultiParameter::BalancePart3
            | MultiParameter::BalancePart4 => RangeInclusive::new(-128, 128),

            // FX
            MultiParameter::FXLength
            | MultiParameter::FXFeedback
            | MultiParameter::FXMix
            | MultiParameter::FXSpeed
            | MultiParameter::FXDepth => RangeInclusive::new(0, 255),
            MultiParameter::FXMode => RangeInclusive::new(0, 4),
        }
    }

    /// Returns the default value for the parameter.
    pub fn get_default(&self) -> i32 {
        0
    }
}

/// Set of all parameter values.
pub struct ParameterValues {
    /// Sound (preset) parameter values
    pub sound: HashMap<SoundParameter, i32>,

    /// Multi parameter values
    pub multi: HashMap<MultiParameter, i32>,
}

impl ParameterValues {
    /// Returns a new initialized struct.
    pub fn new() -> Self {
        Self {
            sound: HashMap::<SoundParameter, i32>::with_capacity(128),
            multi: HashMap::<MultiParameter, i32>::with_capacity(32),
        }
    }

    /// Returns the current value of a parameter.
    pub fn get_value(&self, param: Parameter) -> i32 {
        match param {
            Parameter::Sound(param) => *self.sound.get(&param).unwrap_or(&param.get_default()),
            Parameter::Multi(param) => *self.multi.get(&param).unwrap_or(&param.get_default()),
        }
    }
}
