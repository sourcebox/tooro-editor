use crate::params::{SoundParameter, SoundParameterValues};

// Service ids
pub const SERVICE_MULTI_REQUEST: u8 = 0x01;
pub const SERVICE_PRESET_REQUEST: u8 = 0x02;
pub const SERVICE_MULTI_DUMP: u8 = 0x11;
pub const SERVICE_PRESET_DUMP: u8 = 0x12;

// Total dump lengths in bytes (incl. 0xF0 & 0xF7)
pub const MULTI_DUMP_LENGTH: usize = 104;
pub const PRESET_DUMP_LENGTH: usize = 264;

/// Unpack the data and return a vector of it
///
/// - `data`    Slice of 7-bit sysex payload
pub fn unpack_data(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut cursor = 0;
    let blocks = data.len() / 5;

    for _ in 0..blocks {
        let tops = data[cursor + 4];
        result.push(data[cursor] | ((tops << 7) & 0x80));
        cursor += 1;
        result.push(data[cursor] | ((tops << 6) & 0x80));
        cursor += 1;
        result.push(data[cursor] | ((tops << 5) & 0x80));
        cursor += 1;
        result.push(data[cursor] | ((tops << 4) & 0x80));
        cursor += 2;
    }

    result
}

/// Update all sound parameters according to sysex data
///
/// - `params`  Parameter map to be updated
/// - `values`  Raw values from unpacked sysex data
pub fn update_sound_params(params: &mut SoundParameterValues, values: &Vec<u8>) {
    // Osc 1
    params.insert(SoundParameter::Osc1Wave, value_from_index(values, 0) / 4);
    params.insert(SoundParameter::Osc1Coarse, value_from_index(values, 2));
    params.insert(
        SoundParameter::Osc1FMAmount,
        value_from_index(values, 4) / 4,
    );
    params.insert(SoundParameter::Osc1Level, value_from_index(values, 6) / 4);
    params.insert(SoundParameter::Osc1Table, value_from_index(values, 8));
    params.insert(SoundParameter::Osc1Fine, value_from_index(values, 10));
    params.insert(SoundParameter::Osc1FMRate, value_from_index(values, 12) / 4);
    params.insert(SoundParameter::Osc1Sync, value_from_index(values, 14) / 4);

    // Osc 2
    params.insert(SoundParameter::Osc2Wave, value_from_index(values, 16) / 4);
    params.insert(SoundParameter::Osc2Coarse, value_from_index(values, 18));
    params.insert(
        SoundParameter::Osc2FMAmount,
        value_from_index(values, 20) / 4,
    );
    params.insert(SoundParameter::Osc2Level, value_from_index(values, 22) / 4);
    params.insert(SoundParameter::Osc2Table, value_from_index(values, 24));
    params.insert(SoundParameter::Osc2Fine, value_from_index(values, 26));
    params.insert(SoundParameter::Osc2FMRate, value_from_index(values, 28) / 4);
    params.insert(SoundParameter::Osc2Sync, value_from_index(values, 30) / 4);

    // Extra
    params.insert(SoundParameter::ExtraNoise, value_from_index(values, 32) / 4);
    params.insert(
        SoundParameter::ExtraRingMod,
        value_from_index(values, 34) / 4,
    );

    // Shaper
    params.insert(
        SoundParameter::ShaperCutoff,
        value_from_index(values, 36) / 4,
    );
    params.insert(
        SoundParameter::ShaperResonance,
        value_from_index(values, 38) / 4,
    );
    params.insert(
        SoundParameter::ShaperEnvAAmount,
        value_from_index(values, 40) / 4,
    );
    params.insert(SoundParameter::ShaperTrack, value_from_index(values, 42));
    params.insert(SoundParameter::ShaperMode, value_from_index(values, 44));
    params.insert(
        SoundParameter::ShaperLFO2Amount,
        value_from_index(values, 46) / 4,
    );

    // Filter
    params.insert(
        SoundParameter::FilterCutoff,
        value_from_index(values, 48) / 4,
    );
    params.insert(
        SoundParameter::FilterResonance,
        value_from_index(values, 50) / 4,
    );
    params.insert(
        SoundParameter::FilterEnvFAmount,
        value_from_index(values, 52) / 4,
    );
    params.insert(SoundParameter::FilterTrack, value_from_index(values, 54));
    params.insert(
        SoundParameter::FilterAfter,
        value_from_index(values, 56) / 4,
    );
    params.insert(
        SoundParameter::FilterLFO1Amount,
        value_from_index(values, 58) / 4,
    );

    // Env F
    params.insert(SoundParameter::EnvFAttack, value_from_index(values, 60) / 4);
    params.insert(SoundParameter::EnvFDecay, value_from_index(values, 62) / 4);
    params.insert(
        SoundParameter::EnvFSustain,
        value_from_index(values, 64) / 4,
    );
    params.insert(
        SoundParameter::EnvFRelease,
        value_from_index(values, 66) / 4,
    );
    params.insert(SoundParameter::EnvFVelo, value_from_index(values, 68) / 4);
    params.insert(SoundParameter::EnvFHold, value_from_index(values, 70) / 4);
    params.insert(SoundParameter::EnvFAfter, value_from_index(values, 72) / 4);
    params.insert(SoundParameter::EnvFTrigger, value_from_index(values, 74));

    // Env A
    params.insert(SoundParameter::EnvAAttack, value_from_index(values, 76) / 4);
    params.insert(SoundParameter::EnvADecay, value_from_index(values, 78) / 4);
    params.insert(
        SoundParameter::EnvASustain,
        value_from_index(values, 80) / 4,
    );
    params.insert(
        SoundParameter::EnvARelease,
        value_from_index(values, 82) / 4,
    );
    params.insert(SoundParameter::EnvAVelo, value_from_index(values, 84) / 4);
    params.insert(SoundParameter::EnvAHold, value_from_index(values, 86) / 4);
    params.insert(SoundParameter::EnvAAfter, value_from_index(values, 88) / 4);
    params.insert(SoundParameter::EnvATrigger, value_from_index(values, 90));

    // LFO 1
    params.insert(SoundParameter::LFO1Shape, value_from_index(values, 92));
    params.insert(SoundParameter::LFO1Speed, value_from_index(values, 94) / 4);
    params.insert(SoundParameter::LFO1Rise, value_from_index(values, 96) / 4);
    params.insert(SoundParameter::LFO1Phase, value_from_index(values, 98));

    // LFO 2
    params.insert(SoundParameter::LFO2Shape, value_from_index(values, 100));
    params.insert(SoundParameter::LFO2Speed, value_from_index(values, 102) / 4);
    params.insert(SoundParameter::LFO2Rise, value_from_index(values, 104) / 4);
    params.insert(SoundParameter::LFO2Phase, value_from_index(values, 106));

    // Arpeggiator
    params.insert(SoundParameter::ArpMode, value_from_index(values, 108));
    params.insert(SoundParameter::ArpGrid, value_from_index(values, 110));
    params.insert(
        SoundParameter::ArpTempo,
        rescale(value_from_index(values, 112), 0, 1024, 20, 199),
    );
    params.insert(SoundParameter::ArpHold, value_from_index(values, 114));

    // Amplifier
    params.insert(SoundParameter::AmpLevel, value_from_index(values, 116) / 4);
    params.insert(SoundParameter::AmpPan, value_from_index(values, 118) / 4);

    // Modulations
    params.insert(
        SoundParameter::ModEnvFAmount,
        value_from_index(values, 120) / 4,
    );
    params.insert(SoundParameter::ModEnvFTarget, value_from_index(values, 122));
    params.insert(
        SoundParameter::ModEnvAAmount,
        value_from_index(values, 124) / 4,
    );
    params.insert(SoundParameter::ModEnvATarget, value_from_index(values, 126));
    params.insert(
        SoundParameter::ModLFO1Amount,
        value_from_index(values, 128) / 4,
    );
    params.insert(SoundParameter::ModLFO1Target, value_from_index(values, 130));
    params.insert(
        SoundParameter::ModLFO2Amount,
        value_from_index(values, 132) / 4,
    );
    params.insert(SoundParameter::ModLFO2Target, value_from_index(values, 134));
    params.insert(
        SoundParameter::ModModwheelAmount,
        value_from_index(values, 136) / 4,
    );
    params.insert(
        SoundParameter::ModModwheelTarget,
        value_from_index(values, 138),
    );
    params.insert(
        SoundParameter::ModPitchbendAmount,
        value_from_index(values, 140) / 4,
    );
    params.insert(
        SoundParameter::ModPitchbendTarget,
        value_from_index(values, 142),
    );
    params.insert(
        SoundParameter::ModVelocityAmount,
        value_from_index(values, 144) / 4,
    );
    params.insert(
        SoundParameter::ModVelocityTarget,
        value_from_index(values, 146),
    );
    params.insert(
        SoundParameter::ModAftertouchAmount,
        value_from_index(values, 148) / 4,
    );
    params.insert(
        SoundParameter::ModAftertouchTarget,
        value_from_index(values, 150),
    );

    // Misc
    params.insert(SoundParameter::Tune, value_from_index(values, 168));
    params.insert(SoundParameter::BendRange, value_from_index(values, 172));
}

/// Return parameter value as i32 from values vector addressed by index
///
/// - 'index`   Start index in values vector
fn value_from_index(values: &Vec<u8>, index: usize) -> i32 {
    i16::from_le_bytes([values[index], values[index + 1]]) as i32
}

fn rescale(value: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
