use iced::{pick_list, Container, Length, PickList, Row, Text};

use crate::messages::Message;
use crate::params::SoundParameter;
use crate::style;

pub fn arp_mode_list<'a>(
    label: &'a str,
    state: &'a mut pick_list::State<ArpMode>,
    sound_param: SoundParameter,
    value: i32,
) -> Container<'a, Message> {
    let value = match value {
        0 => Some(ArpMode::Off),
        1 => Some(ArpMode::Up),
        2 => Some(ArpMode::Down),
        3 => Some(ArpMode::Random),
        4 => Some(ArpMode::Up2),
        5 => Some(ArpMode::Down2),
        6 => Some(ArpMode::Up4),
        7 => Some(ArpMode::Down4),
        _ => None,
    };
    let pick_list = PickList::new(state, &ArpMode::ALL[..], value, move |v| {
        Message::SoundParameterChange(sound_param, v as i32)
    })
    .style(style::PickList)
    .text_size(style::LIST_ITEM_TEXT_SIZE);

    Container::new(
        Row::new()
            .push(
                Text::new(label)
                    .size(style::PARAM_LABEL_TEXT_SIZE)
                    .width(Length::Units(style::PARAM_LABEL_WIDTH)),
            )
            .push(pick_list),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArpMode {
    Off,
    Up,
    Down,
    Random,
    Up2,
    Down2,
    Up4,
    Down4,
}

impl ArpMode {
    const ALL: [ArpMode; 8] = [
        ArpMode::Off,
        ArpMode::Up,
        ArpMode::Down,
        ArpMode::Random,
        ArpMode::Up2,
        ArpMode::Down2,
        ArpMode::Up4,
        ArpMode::Down4,
    ];
}

impl std::fmt::Display for ArpMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArpMode::Off => "Off",
                ArpMode::Up => "Up",
                ArpMode::Down => "Down",
                ArpMode::Random => "Random",
                ArpMode::Up2 => "Up 2",
                ArpMode::Down2 => "Down 2",
                ArpMode::Up4 => "Up 4",
                ArpMode::Down4 => "Down 4",
            }
        )
    }
}
