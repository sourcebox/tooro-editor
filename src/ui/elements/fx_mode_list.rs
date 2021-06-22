//! Dropdown menu for the multi fx modes

use iced::{pick_list, Column, Container, Length, PickList, Row, Text};

use crate::messages::Message;
use crate::params::MultiParameter;
use crate::style;

pub fn fx_mode_list<'a>(
    label: &'a str,
    state: &'a mut pick_list::State<FXMode>,
    multi_param: MultiParameter,
    value: i32,
) -> Container<'a, Message> {
    let value = match value {
        0 => Some(FXMode::Off),
        1 => Some(FXMode::MonoDelay),
        2 => Some(FXMode::ChorusFlanger),
        3 => Some(FXMode::StereoDelay),
        _ => None,
    };
    let pick_list = PickList::new(state, &FXMode::ALL[..], value, move |v| {
        Message::MultiParameterChange(multi_param, v as i32)
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
pub enum FXMode {
    Off,
    MonoDelay,
    ChorusFlanger,
    StereoDelay,
}

impl FXMode {
    const ALL: [FXMode; 4] = [
        FXMode::Off,
        FXMode::MonoDelay,
        FXMode::ChorusFlanger,
        FXMode::StereoDelay,
    ];
}

impl std::fmt::Display for FXMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FXMode::Off => "Off",
                FXMode::MonoDelay => "Mono Delay",
                FXMode::ChorusFlanger => "Chorus/Flanger",
                FXMode::StereoDelay => "Stereo Delay",
            }
        )
    }
}
