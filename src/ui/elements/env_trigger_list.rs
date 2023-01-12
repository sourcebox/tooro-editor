//! Dropdown menu for the envelope trigger modes

use iced::widget::{Column, Container, PickList, Row, Text};
use iced::Length;

use crate::messages::Message;
use crate::params::SoundParameter;
use crate::style;

pub fn env_trigger_list(
    label: &str,
    sound_param: SoundParameter,
    value: i32,
) -> Container<Message> {
    let value = match value {
        0 => Some(EnvTrigger::Always),
        1 => Some(EnvTrigger::Never),
        2 => Some(EnvTrigger::Continue),
        _ => None,
    };
    let pick_list = PickList::new(&EnvTrigger::ALL[..], value, move |v| {
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
pub enum EnvTrigger {
    Always,
    Never,
    Continue,
}

impl EnvTrigger {
    const ALL: [EnvTrigger; 3] = [EnvTrigger::Always, EnvTrigger::Never, EnvTrigger::Continue];
}

impl std::fmt::Display for EnvTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EnvTrigger::Always => "Always",
                EnvTrigger::Never => "Never",
                EnvTrigger::Continue => "Continue",
            }
        )
    }
}
