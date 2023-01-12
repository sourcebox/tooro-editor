//! Dropdown menu for the LFO phase values

use iced::widget::{Column, Container, PickList, Row, Text};
use iced::Length;

use crate::messages::Message;
use crate::params::SoundParameter;
use crate::style;

pub fn lfo_phase_list<'a>(
    label: &'a str,
    sound_param: SoundParameter,
    value: i32,
) -> Container<'a, Message> {
    let value = match value {
        0 => Some(LFOPhase::Free),
        1 => Some(LFOPhase::Random),
        2 => Some(LFOPhase::Phase0),
        3 => Some(LFOPhase::Phase90),
        4 => Some(LFOPhase::Phase180),
        5 => Some(LFOPhase::Phase270),

        _ => None,
    };
    let pick_list = PickList::new(&LFOPhase::ALL[..], value, move |v| {
        Message::SoundParameterChange(sound_param, v as i32)
    })
    .text_size(style::LIST_ITEM_TEXT_SIZE);

    Container::new(
        Row::new()
            .push(
                Column::new()
                    .push(
                        Text::new(label)
                            .size(style::PARAM_LABEL_TEXT_SIZE)
                            .width(Length::Units(style::PARAM_LABEL_WIDTH)),
                    )
                    .padding([4, 0, 0, 0]),
            )
            .push(pick_list),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LFOPhase {
    Free,
    Random,
    Phase0,
    Phase90,
    Phase180,
    Phase270,
}

impl LFOPhase {
    const ALL: [LFOPhase; 6] = [
        LFOPhase::Free,
        LFOPhase::Random,
        LFOPhase::Phase0,
        LFOPhase::Phase90,
        LFOPhase::Phase180,
        LFOPhase::Phase270,
    ];
}

impl std::fmt::Display for LFOPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LFOPhase::Free => "Free",
                LFOPhase::Random => "Random",
                LFOPhase::Phase0 => "0°",
                LFOPhase::Phase90 => "90°",
                LFOPhase::Phase180 => "180°",
                LFOPhase::Phase270 => "270°",
            }
        )
    }
}
