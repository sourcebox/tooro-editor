//! Dropdown menu for the shaper modes

use iced::widget::{Column, Container, PickList, Row, Text};
use iced::Length;

use crate::messages::Message;
use crate::params::SoundParameter;
use crate::style;

pub fn shaper_mode_list(
    label: &str,
    sound_param: SoundParameter,
    value: i32,
) -> Container<Message> {
    let value = match value {
        0 => Some(ShaperMode::Lowpass),
        1 => Some(ShaperMode::Bandpass),
        2 => Some(ShaperMode::Highpass),
        _ => None,
    };
    let pick_list = PickList::new(&ShaperMode::ALL[..], value, move |v| {
        Message::SoundParameterChange(sound_param, v as i32)
    })
    .style(style::PickList)
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
pub enum ShaperMode {
    Lowpass,
    Bandpass,
    Highpass,
}

impl ShaperMode {
    const ALL: [ShaperMode; 3] = [
        ShaperMode::Lowpass,
        ShaperMode::Bandpass,
        ShaperMode::Highpass,
    ];
}

impl std::fmt::Display for ShaperMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShaperMode::Lowpass => "Low-pass",
                ShaperMode::Bandpass => "Band-pass",
                ShaperMode::Highpass => "High-pass",
            }
        )
    }
}
