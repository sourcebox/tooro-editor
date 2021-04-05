use iced::{pick_list, Container, Length, PickList, Row, Text};

use crate::messages::Message;
use crate::params::SoundParameter;
use crate::style;

pub fn mod_target_list<'a>(
    label: &'a str,
    state: &'a mut pick_list::State<ModTarget>,
    sound_param: SoundParameter,
    value: i32,
) -> Container<'a, Message> {
    let value = match value {
        0 => Some(ModTarget::Osc1Wave),
        1 => Some(ModTarget::Osc2Wave),
        2 => Some(ModTarget::Osc1Pitch),
        3 => Some(ModTarget::Osc2Pitch),
        4 => Some(ModTarget::Osc1FMAmount),
        5 => Some(ModTarget::Osc2FMAmount),
        6 => Some(ModTarget::Osc1FMRate),
        7 => Some(ModTarget::Osc2FMRate),
        8 => Some(ModTarget::Osc1Sync),
        9 => Some(ModTarget::Osc2Sync),
        10 => Some(ModTarget::Osc1Level),
        11 => Some(ModTarget::Osc2Level),
        12 => Some(ModTarget::ExtraRingMod),
        13 => Some(ModTarget::ExtraRingMod),
        14 => Some(ModTarget::FilterCutoff),
        15 => Some(ModTarget::ShaperCutoff),
        16 => Some(ModTarget::FilterResonance),
        17 => Some(ModTarget::ShaperResonance),
        18 => Some(ModTarget::LFO1Speed),
        19 => Some(ModTarget::LFO2Speed),
        20 => Some(ModTarget::AmpLevel),
        21 => Some(ModTarget::AmpPan),
        _ => None,
    };
    let pick_list = PickList::new(state, &ModTarget::ALL[..], value, move |v| {
        Message::SoundParameterChange(sound_param, v as i32)
    })
    .style(style::PickList)
    .text_size(16);

    Container::new(
        Row::new()
            .push(Text::new(label).size(16).width(Length::Units(80)))
            .push(pick_list),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModTarget {
    Osc1Wave,
    Osc2Wave,
    Osc1Pitch,
    Osc2Pitch,
    Osc1FMAmount,
    Osc2FMAmount,
    Osc1FMRate,
    Osc2FMRate,
    Osc1Sync,
    Osc2Sync,
    Osc1Level,
    Osc2Level,
    ExtraNoise,
    ExtraRingMod,
    FilterCutoff,
    ShaperCutoff,
    FilterResonance,
    ShaperResonance,
    LFO1Speed,
    LFO2Speed,
    AmpLevel,
    AmpPan,
}

impl ModTarget {
    const ALL: [ModTarget; 22] = [
        ModTarget::Osc1Wave,
        ModTarget::Osc2Wave,
        ModTarget::Osc1Pitch,
        ModTarget::Osc2Pitch,
        ModTarget::Osc1FMAmount,
        ModTarget::Osc2FMAmount,
        ModTarget::Osc1FMRate,
        ModTarget::Osc2FMRate,
        ModTarget::Osc1Sync,
        ModTarget::Osc2Sync,
        ModTarget::Osc1Level,
        ModTarget::Osc2Level,
        ModTarget::ExtraNoise,
        ModTarget::ExtraRingMod,
        ModTarget::FilterCutoff,
        ModTarget::ShaperCutoff,
        ModTarget::FilterResonance,
        ModTarget::ShaperResonance,
        ModTarget::LFO1Speed,
        ModTarget::LFO2Speed,
        ModTarget::AmpLevel,
        ModTarget::AmpPan,
    ];
}

impl std::fmt::Display for ModTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModTarget::Osc1Wave => "Osc 1 Wave",
                ModTarget::Osc2Wave => "Osc 2 Wave",
                ModTarget::Osc1Pitch => "Osc 1 Pitch",
                ModTarget::Osc2Pitch => "Osc 2 Pitch",
                ModTarget::Osc1FMAmount => "Osc 1 FM Amount",
                ModTarget::Osc2FMAmount => "Osc 2 FM Amount",
                ModTarget::Osc1FMRate => "Osc 1 FM Rate",
                ModTarget::Osc2FMRate => "Osc 2 FM Rate",
                ModTarget::Osc1Sync => "Osc 1 Sync",
                ModTarget::Osc2Sync => "Osc 2 Sync",
                ModTarget::Osc1Level => "Osc 1 Level",
                ModTarget::Osc2Level => "Osc 2 Level",
                ModTarget::ExtraNoise => "Extra Noise",
                ModTarget::ExtraRingMod => "Extra RingMod",
                ModTarget::FilterCutoff => "Filter Cutoff",
                ModTarget::ShaperCutoff => "Shaper Cutoff",
                ModTarget::FilterResonance => "Filter Resonance",
                ModTarget::ShaperResonance => "Shaper Resonance",
                ModTarget::LFO1Speed => "LFO 1 Speed",
                ModTarget::LFO2Speed => "LFO 2 Speed",
                ModTarget::AmpLevel => "Amp Level",
                ModTarget::AmpPan => "Amp Pan",
            }
        )
    }
}
