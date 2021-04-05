use crate::params::SoundParameter;

/// Converts a sound parameter to its corresponding MIDI CC
pub fn sound_param_to_cc(param: &SoundParameter, value: i32) -> Option<(u8, u8)> {
    let cc_num = match param {
        // Osc 1
        SoundParameter::Osc1Wave => Some(70),
        SoundParameter::Osc1Coarse => Some(21),
        SoundParameter::Osc1FMAmount => Some(22),
        SoundParameter::Osc1Level => Some(23),
        SoundParameter::Osc1Table => Some(24),
        SoundParameter::Osc1Fine => Some(25),
        SoundParameter::Osc1FMRate => Some(26),
        SoundParameter::Osc1Sync => Some(27),

        // Osc 2
        SoundParameter::Osc2Wave => Some(77),
        SoundParameter::Osc2Coarse => Some(31),
        SoundParameter::Osc2FMAmount => Some(32),
        SoundParameter::Osc2Level => Some(33),
        SoundParameter::Osc2Table => Some(34),
        SoundParameter::Osc2Fine => Some(35),
        SoundParameter::Osc2FMRate => Some(36),
        SoundParameter::Osc2Sync => Some(37),

        // Extra
        SoundParameter::ExtraNoise => Some(78),
        SoundParameter::ExtraRingMod => Some(79),

        // Filter
        SoundParameter::FilterCutoff => Some(74),
        SoundParameter::FilterResonance => Some(71),
        SoundParameter::FilterEnvFAmount => Some(54),
        SoundParameter::FilterTrack => Some(55),
        SoundParameter::FilterAfter => Some(56),
        SoundParameter::FilterLFO1Amount => Some(57),

        // Shaper
        SoundParameter::ShaperCutoff => Some(75),
        SoundParameter::ShaperResonance => Some(76),
        SoundParameter::ShaperEnvAAmount => Some(58),
        SoundParameter::ShaperTrack => Some(59),
        SoundParameter::ShaperMode => Some(60),
        SoundParameter::ShaperLFO2Amount => Some(61),

        // Env F
        SoundParameter::EnvFAttack => Some(46),
        SoundParameter::EnvFDecay => Some(47),
        SoundParameter::EnvFSustain => Some(48),
        SoundParameter::EnvFRelease => Some(49),
        SoundParameter::EnvFVelo => Some(51),
        SoundParameter::EnvFHold => Some(50),
        SoundParameter::EnvFAfter => Some(52),
        SoundParameter::EnvFTrigger => Some(53),

        // Env A
        SoundParameter::EnvAAttack => Some(73),
        SoundParameter::EnvADecay => Some(40),
        SoundParameter::EnvASustain => Some(41),
        SoundParameter::EnvARelease => Some(72),
        SoundParameter::EnvAVelo => Some(43),
        SoundParameter::EnvAHold => Some(42),
        SoundParameter::EnvAAfter => Some(44),
        SoundParameter::EnvATrigger => Some(45),

        // LFO 1
        SoundParameter::LFO1Shape => Some(102),
        SoundParameter::LFO1Speed => Some(103),
        SoundParameter::LFO1Rise => Some(104),
        SoundParameter::LFO1Phase => Some(105),

        // LFO 2
        SoundParameter::LFO2Shape => Some(106),
        SoundParameter::LFO2Speed => Some(107),
        SoundParameter::LFO2Rise => Some(108),
        SoundParameter::LFO2Phase => Some(109),

        // Arpeggiator
        SoundParameter::ArpMode => Some(110),
        SoundParameter::ArpGrid => Some(111),
        SoundParameter::ArpTempo => Some(112),
        SoundParameter::ArpHold => Some(113),

        // Amplifier
        SoundParameter::AmpLevel => Some(80),
        SoundParameter::AmpPan => Some(81),

        // Modulations
        SoundParameter::ModEnvFAmount => Some(17),
        SoundParameter::ModEnvAAmount => Some(18),
        SoundParameter::ModLFO1Amount => Some(19),
        SoundParameter::ModLFO2Amount => Some(20),
        SoundParameter::ModModwheelAmount => Some(87),
        SoundParameter::ModPitchAmount => Some(88),
        SoundParameter::ModVelocityAmount => Some(89),
        SoundParameter::ModAftertouchAmount => Some(90),

        // Misc
        SoundParameter::BendRange => Some(84),
        SoundParameter::Tune => Some(85),

        _ => None,
    };

    if cc_num.is_none() {
        return None;
    }

    let cc_value = (match param {
        // Default
        _ => {
            let range = param.get_range();
            rescale(value, *range.start(), *range.end(), 0, 127)
        }
    } as u8)
        .clamp(0, 127);

    Some((cc_num.unwrap(), cc_value))
}

/// Converts a MIDI CC to its corresponding sound parameter
pub fn cc_to_sound_param(cc_num: u8, cc_value: u8) -> Option<(SoundParameter, i32)> {
    let param = match cc_num {
        // Osc 1
        70 => Some(SoundParameter::Osc1Wave),
        21 => Some(SoundParameter::Osc1Coarse),
        22 => Some(SoundParameter::Osc1FMAmount),
        23 => Some(SoundParameter::Osc1Level),
        24 => Some(SoundParameter::Osc1Table),
        25 => Some(SoundParameter::Osc1Fine),
        26 => Some(SoundParameter::Osc1FMRate),
        27 => Some(SoundParameter::Osc1Sync),

        // Osc 2
        77 => Some(SoundParameter::Osc2Wave),
        31 => Some(SoundParameter::Osc2Coarse),
        32 => Some(SoundParameter::Osc2FMAmount),
        33 => Some(SoundParameter::Osc2Level),
        34 => Some(SoundParameter::Osc2Table),
        35 => Some(SoundParameter::Osc2Fine),
        36 => Some(SoundParameter::Osc2FMRate),
        37 => Some(SoundParameter::Osc2Sync),

        // Extra
        78 => Some(SoundParameter::ExtraNoise),
        79 => Some(SoundParameter::ExtraRingMod),

        // Filter
        74 => Some(SoundParameter::FilterCutoff),
        71 => Some(SoundParameter::FilterResonance),
        54 => Some(SoundParameter::FilterEnvFAmount),
        55 => Some(SoundParameter::FilterTrack),
        56 => Some(SoundParameter::FilterAfter),
        57 => Some(SoundParameter::FilterLFO1Amount),

        // Shaper
        75 => Some(SoundParameter::ShaperCutoff),
        76 => Some(SoundParameter::ShaperResonance),
        58 => Some(SoundParameter::ShaperEnvAAmount),
        59 => Some(SoundParameter::ShaperTrack),
        60 => Some(SoundParameter::ShaperMode),
        61 => Some(SoundParameter::ShaperLFO2Amount),

        // Env F
        46 => Some(SoundParameter::EnvFAttack),
        47 => Some(SoundParameter::EnvFDecay),
        48 => Some(SoundParameter::EnvFSustain),
        49 => Some(SoundParameter::EnvFRelease),
        50 => Some(SoundParameter::EnvFVelo),
        51 => Some(SoundParameter::EnvFHold),
        52 => Some(SoundParameter::EnvFAfter),
        53 => Some(SoundParameter::EnvFTrigger),

        // Env A
        73 => Some(SoundParameter::EnvAAttack),
        40 => Some(SoundParameter::EnvADecay),
        41 => Some(SoundParameter::EnvASustain),
        72 => Some(SoundParameter::EnvARelease),
        43 => Some(SoundParameter::EnvAVelo),
        42 => Some(SoundParameter::EnvAHold),
        44 => Some(SoundParameter::EnvAAfter),
        45 => Some(SoundParameter::EnvATrigger),

        // LFO 1
        102 => Some(SoundParameter::LFO1Shape),
        103 => Some(SoundParameter::LFO1Speed),
        104 => Some(SoundParameter::LFO1Rise),
        105 => Some(SoundParameter::LFO1Phase),

        // LFO 2
        106 => Some(SoundParameter::LFO2Shape),
        107 => Some(SoundParameter::LFO2Speed),
        108 => Some(SoundParameter::LFO2Rise),
        109 => Some(SoundParameter::LFO2Phase),

        // Arpeggiator
        110 => Some(SoundParameter::ArpMode),
        111 => Some(SoundParameter::ArpGrid),
        112 => Some(SoundParameter::ArpTempo),
        113 => Some(SoundParameter::ArpHold),

        // Amplifier
        80 => Some(SoundParameter::AmpLevel),
        81 => Some(SoundParameter::AmpPan),

        // Modulations
        17 => Some(SoundParameter::ModEnvFAmount),
        18 => Some(SoundParameter::ModEnvAAmount),
        19 => Some(SoundParameter::ModLFO1Amount),
        20 => Some(SoundParameter::ModLFO2Amount),
        87 => Some(SoundParameter::ModModwheelAmount),
        88 => Some(SoundParameter::ModPitchAmount),
        89 => Some(SoundParameter::ModVelocityAmount),
        90 => Some(SoundParameter::ModAftertouchAmount),

        // Misc
        84 => Some(SoundParameter::BendRange),
        85 => Some(SoundParameter::Tune),

        // Unknown
        _ => None,
    };

    if param.is_none() {
        return None;
    }

    let param = param.unwrap();

    let value = match param {
        // Default
        _ => {
            let range = param.get_range();
            rescale(cc_value as i32, 0, 127, *range.start(), *range.end())
        }
    };

    Some((param, value))
}

fn rescale(value: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
