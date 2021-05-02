use crate::params::{MultiParameter, MultiParameterValues, SoundParameter, SoundParameterValues};

// Service ids
pub const SERVICE_MULTI_REQUEST: u8 = 0x01;
pub const SERVICE_PRESET_REQUEST: u8 = 0x02;
pub const SERVICE_PRESET_PARAMETER_REQUEST: u8 = 0x03;
pub const SERVICE_MULTI_DUMP: u8 = 0x11;
pub const SERVICE_PRESET_DUMP: u8 = 0x12;
pub const SERVICE_PRESET_PARAMETER_DUMP: u8 = 0x13;

// Total dump lengths in bytes (incl. 0xF0 & 0xF7)
pub const MULTI_DUMP_LENGTH: usize = 104;
pub const PRESET_DUMP_LENGTH: usize = 264;
pub const PRESET_PARAMETER_DUMP_LENGTH: usize = 8;

/// Return message for multi request
///
/// - `multi_id`   Multi id, either 0..9 or 0x7F
pub fn multi_request(multi_id: u8) -> Vec<u8> {
    vec![0xF0, SERVICE_MULTI_REQUEST, multi_id, 0xF7]
}

/// Return message for preset request
///
/// - `preset_id`   Preset id, either 0..99 or 0x70..0x73
pub fn preset_request(preset_id: u8) -> Vec<u8> {
    vec![0xF0, SERVICE_PRESET_REQUEST, preset_id, 0xF7]
}

/// Return message for preset parameter dump
///
/// - `preset_id`   Preset id 0x70..0x73
/// - `param`       Sound parameter enum value
/// - `value`       Sound parameter value
pub fn preset_param_dump(preset_id: u8, param: &SoundParameter, value: i32) -> Vec<u8> {
    let (id, value) = match param {
        // Osc 1
        SoundParameter::Osc1Wave => (0, value * 4),
        SoundParameter::Osc1Coarse => (1, value),
        SoundParameter::Osc1FMAmount => (2, value * 4),
        SoundParameter::Osc1Level => (3, value * 4),
        SoundParameter::Osc1Table => (4, value),
        SoundParameter::Osc1Fine => (5, value),
        SoundParameter::Osc1FMRate => (6, value * 4),
        SoundParameter::Osc1Sync => (7, value * 4),

        // Osc 2
        SoundParameter::Osc2Wave => (8, value * 4),
        SoundParameter::Osc2Coarse => (9, value),
        SoundParameter::Osc2FMAmount => (10, value * 4),
        SoundParameter::Osc2Level => (11, value * 4),
        SoundParameter::Osc2Table => (12, value),
        SoundParameter::Osc2Fine => (13, value),
        SoundParameter::Osc2FMRate => (14, value * 4),
        SoundParameter::Osc2Sync => (15, value * 4),

        // Extra
        SoundParameter::ExtraNoise => (16, value * 4),
        SoundParameter::ExtraRingMod => (17, value * 4),

        // Shaper
        SoundParameter::ShaperCutoff => (18, value * 4),
        SoundParameter::ShaperResonance => (19, value * 4),
        SoundParameter::ShaperEnvAAmount => (20, value * 4),
        SoundParameter::ShaperTrack => (21, value),
        SoundParameter::ShaperMode => (22, value),
        SoundParameter::ShaperLFO2Amount => (23, value * 4),

        // Filter
        SoundParameter::FilterCutoff => (24, value * 4),
        SoundParameter::FilterResonance => (25, value * 4),
        SoundParameter::FilterEnvFAmount => (26, value * 4),
        SoundParameter::FilterTrack => (27, value),
        SoundParameter::FilterAfter => (28, value * 4),
        SoundParameter::FilterLFO1Amount => (29, value * 4),

        // Env F
        SoundParameter::EnvFAttack => (30, value * 4),
        SoundParameter::EnvFDecay => (31, value * 4),
        SoundParameter::EnvFSustain => (32, value * 4),
        SoundParameter::EnvFRelease => (33, value * 4),
        SoundParameter::EnvFVelo => (34, value * 4),
        SoundParameter::EnvFHold => (35, value * 4),
        SoundParameter::EnvFAfter => (36, value * 4),
        SoundParameter::EnvFTrigger => (37, value),

        // Env A
        SoundParameter::EnvAAttack => (38, value * 4),
        SoundParameter::EnvADecay => (39, value * 4),
        SoundParameter::EnvASustain => (40, value * 4),
        SoundParameter::EnvARelease => (41, value * 4),
        SoundParameter::EnvAVelo => (42, value * 4),
        SoundParameter::EnvAHold => (43, value * 4),
        SoundParameter::EnvAAfter => (44, value * 4),
        SoundParameter::EnvATrigger => (45, value),

        // LFO 1
        SoundParameter::LFO1Shape => (46, value),
        SoundParameter::LFO1Speed => (47, value * 4),
        SoundParameter::LFO1Rise => (48, value * 4),
        SoundParameter::LFO1Phase => (49, value),

        // LFO 2
        SoundParameter::LFO2Shape => (50, value),
        SoundParameter::LFO2Speed => (51, value * 4),
        SoundParameter::LFO2Rise => (52, value * 4),
        SoundParameter::LFO2Phase => (53, value),

        // Arpeggiator
        SoundParameter::ArpMode => (54, value),
        SoundParameter::ArpGrid => (55, value),
        SoundParameter::ArpTempo => (56, rescale(value, 20, 199, 0, 1024)),
        SoundParameter::ArpHold => (57, value),

        // Amplifier
        SoundParameter::AmpLevel => (58, value * 4),
        SoundParameter::AmpPan => (59, value * 4),

        // Modulations
        SoundParameter::ModEnvFAmount => (60, value * 4),
        SoundParameter::ModEnvFTarget => (61, value),
        SoundParameter::ModEnvAAmount => (62, value * 4),
        SoundParameter::ModEnvATarget => (63, value),
        SoundParameter::ModLFO1Amount => (64, value * 4),
        SoundParameter::ModLFO1Target => (65, value),
        SoundParameter::ModLFO2Amount => (66, value * 4),
        SoundParameter::ModLFO2Target => (67, value),
        SoundParameter::ModModwheelAmount => (68, value * 4),
        SoundParameter::ModModwheelTarget => (69, value),
        SoundParameter::ModPitchAmount => (70, value * 4),
        SoundParameter::ModPitchTarget => (71, value),
        SoundParameter::ModVelocityAmount => (72, value * 4),
        SoundParameter::ModVelocityTarget => (73, value),
        SoundParameter::ModAftertouchAmount => (74, value * 4),
        SoundParameter::ModAftertouchTarget => (75, value),

        // Misc
        SoundParameter::Tune => (84, value),
        SoundParameter::BendRange => (86, value),
        SoundParameter::PolyMode => (87, value),
    };

    let id_low = id & 0x7F;
    let id_high = (id >> 7) & 0x7F;
    let value_low = (value & 0x7F) as u8;
    let value_high = ((value >> 7) & 0x7F) as u8;

    vec![
        0xF0,
        SERVICE_PRESET_PARAMETER_DUMP,
        preset_id,
        id_low,
        id_high,
        value_low,
        value_high,
        0xF7,
    ]
}

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
        SoundParameter::ModPitchAmount,
        value_from_index(values, 140) / 4,
    );
    params.insert(
        SoundParameter::ModPitchTarget,
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
    params.insert(SoundParameter::PolyMode, value_from_index(values, 174));
}

/// Update all multi parameters according to sysex data
///
/// - `params`  Parameter map to be updated
/// - `values`  Raw values from unpacked sysex data
pub fn update_multi_params(params: &mut MultiParameterValues, values: &Vec<u8>) {
    // Preset IDs
    params.insert(MultiParameter::PresetPart1, value_from_index(values, 0));
    params.insert(MultiParameter::PresetPart2, value_from_index(values, 2));
    params.insert(MultiParameter::PresetPart3, value_from_index(values, 4));
    params.insert(MultiParameter::PresetPart4, value_from_index(values, 6));

    // MIDI channels
    params.insert(MultiParameter::ChannelPart1, value_from_index(values, 8));
    params.insert(MultiParameter::ChannelPart2, value_from_index(values, 10));
    params.insert(MultiParameter::ChannelPart3, value_from_index(values, 12));
    params.insert(MultiParameter::ChannelPart4, value_from_index(values, 14));

    // Volumes
    params.insert(
        MultiParameter::VolumePart1,
        value_from_index(values, 16) / 4,
    );
    params.insert(
        MultiParameter::VolumePart2,
        value_from_index(values, 18) / 4,
    );
    params.insert(
        MultiParameter::VolumePart3,
        value_from_index(values, 20) / 4,
    );
    params.insert(
        MultiParameter::VolumePart4,
        value_from_index(values, 22) / 4,
    );

    // Balances
    params.insert(
        MultiParameter::BalancePart1,
        value_from_index(values, 24) / 4,
    );
    params.insert(
        MultiParameter::BalancePart2,
        value_from_index(values, 26) / 4,
    );
    params.insert(
        MultiParameter::BalancePart3,
        value_from_index(values, 28) / 4,
    );
    params.insert(
        MultiParameter::BalancePart4,
        value_from_index(values, 30) / 4,
    );

    // FX
    params.insert(MultiParameter::FXLength, value_from_index(values, 32) / 4);
    params.insert(MultiParameter::FXFeedback, value_from_index(values, 34) / 4);
    params.insert(MultiParameter::FXMix, value_from_index(values, 36) / 4);
    params.insert(MultiParameter::FXMode, value_from_index(values, 38));
    params.insert(MultiParameter::FXSpeed, value_from_index(values, 40) / 4);
    params.insert(MultiParameter::FXDepth, value_from_index(values, 40) / 4);
}

/// Return parameter value as i32 from values vector addressed by index
///
/// - `index`   Start index in values vector
fn value_from_index(values: &Vec<u8>, index: usize) -> i32 {
    i16::from_le_bytes([values[index], values[index + 1]]) as i32
}

fn rescale(value: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
