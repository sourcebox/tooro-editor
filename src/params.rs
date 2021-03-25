use std::collections::HashMap;
use std::ops::RangeInclusive;

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
    ModEnvF,
    ModEnvA,
    ModLFO1,
    ModLFO2,
    ModModwheel,
    ModPitchbend,
    ModVelocity,
    ModAftertouch,

    // Misc
    BendRange,
    Tune,
}

impl SoundParameter {
    pub fn get_range(&self) -> RangeInclusive<i32> {
        match self {
            // Default for bipolar
            SoundParameter::ShaperEnvAAmount
            | SoundParameter::ShaperLFO2Amount
            | SoundParameter::ShaperTrack
            | SoundParameter::FilterEnvFAmount
            | SoundParameter::FilterLFO1Amount
            | SoundParameter::FilterTrack
            | SoundParameter::FilterAfter
            | SoundParameter::AmpPan
            | SoundParameter::Tune => RangeInclusive::new(-64, 63),

            // Speecial ranges
            SoundParameter::Osc1Coarse | SoundParameter::Osc2Coarse => RangeInclusive::new(-36, 36),
            SoundParameter::Osc1Fine | SoundParameter::Osc2Fine => RangeInclusive::new(-100, 100),

            // Lists
            SoundParameter::Osc1Table | SoundParameter::Osc2Table => RangeInclusive::new(0, 10),
            SoundParameter::ShaperMode => RangeInclusive::new(0, 2),
            SoundParameter::EnvATrigger | SoundParameter::EnvFTrigger => RangeInclusive::new(0, 2),
            SoundParameter::LFO1Shape | SoundParameter::LFO2Shape => RangeInclusive::new(0, 7),
            SoundParameter::LFO1Phase | SoundParameter::LFO2Phase => RangeInclusive::new(0, 5),
            SoundParameter::ArpMode => RangeInclusive::new(0, 7),
            SoundParameter::ArpGrid => RangeInclusive::new(0, 9),

            // Boolean
            SoundParameter::ArpHold => RangeInclusive::new(0, 1),

            // Default
            _ => RangeInclusive::new(0, 127),
        }
    }

    pub fn get_default(&self) -> i32 {
        match self {
            _ => 0,
        }
    }
}

pub type SoundParameterValues = HashMap<SoundParameter, i32>;

pub trait GetValue {
    fn get_value(&self, param: SoundParameter) -> i32;
}

impl GetValue for SoundParameterValues {
    fn get_value(&self, param: SoundParameter) -> i32 {
        *self.get(&param).unwrap_or(&param.get_default())
    }
}
