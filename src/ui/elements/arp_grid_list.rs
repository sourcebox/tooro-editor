//! Dropdown menu for the arpeggiator grid values

use iced::widget::{Column, Container, PickList, Row, Text};
use iced::Length;

use crate::messages::Message;
use crate::params::SoundParameter;
use crate::style;

pub fn arp_grid_list(label: &str, sound_param: SoundParameter, value: i32) -> Container<Message> {
    let value = match value {
        0 => Some(ArpGrid::Div48),
        1 => Some(ArpGrid::Div32),
        2 => Some(ArpGrid::Div24),
        3 => Some(ArpGrid::Div16),
        4 => Some(ArpGrid::Div12),
        5 => Some(ArpGrid::Div8),
        6 => Some(ArpGrid::Div6),
        _ => None,
    };
    let pick_list = PickList::new(&ArpGrid::ALL[..], value, move |v| {
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
pub enum ArpGrid {
    Div48,
    Div32,
    Div24,
    Div16,
    Div12,
    Div8,
    Div6,
}

impl ArpGrid {
    const ALL: [ArpGrid; 7] = [
        ArpGrid::Div48,
        ArpGrid::Div32,
        ArpGrid::Div24,
        ArpGrid::Div16,
        ArpGrid::Div12,
        ArpGrid::Div8,
        ArpGrid::Div6,
    ];
}

impl std::fmt::Display for ArpGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArpGrid::Div48 => "1/2",
                ArpGrid::Div32 => "1/2 triplet",
                ArpGrid::Div24 => "1/4",
                ArpGrid::Div16 => "1/4 triplet",
                ArpGrid::Div12 => "1/8",
                ArpGrid::Div8 => "1/8 triplet",
                ArpGrid::Div6 => "1/16",
            }
        )
    }
}
