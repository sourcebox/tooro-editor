//! Dropdown menu for the multi fx modes

use iced::{
    Length, Padding,
    widget::{Container, PickList, container, row, text},
};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues};
use crate::style;

pub fn fx_mode_list<'a>(
    label: &'a str,
    param: Parameter,
    values: &'a ParameterValues,
) -> Container<'a, Message> {
    let value = match values.get_value(param) {
        0 => Some(FXMode::Off),
        1 => Some(FXMode::MonoDelay),
        2 => Some(FXMode::ChorusFlanger),
        3 => Some(FXMode::StereoDelay),
        _ => None,
    };
    let pick_list = PickList::new(&FXMode::ALL[..], value, move |v| {
        Message::ParameterChange(param, v as i32)
    })
    .style(|_, status| style::pick_list(status))
    .text_size(style::LIST_ITEM_TEXT_SIZE)
    .width(Length::Fill);

    container(row![
        container(
            text(label)
                .size(style::PARAM_LABEL_TEXT_SIZE)
                .width(style::PARAM_LABEL_WIDTH),
        )
        .padding(Padding {
            top: 4.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }),
        pick_list
    ])
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
