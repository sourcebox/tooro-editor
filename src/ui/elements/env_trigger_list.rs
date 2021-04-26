use iced::{pick_list, Container, Length, PickList, Row, Text};

use crate::messages::Message;
use crate::params::SoundParameter;
use crate::style;

pub fn env_trigger_list<'a>(
    label: &'a str,
    state: &'a mut pick_list::State<EnvTrigger>,
    sound_param: SoundParameter,
    value: i32,
) -> Container<'a, Message> {
    let value = match value {
        0 => Some(EnvTrigger::Always),
        1 => Some(EnvTrigger::Never),
        2 => Some(EnvTrigger::Continue),
        _ => None,
    };
    let pick_list = PickList::new(state, &EnvTrigger::ALL[..], value, move |v| {
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