//! Dropdown menu for the arpeggiator grid values

use iced::{pick_list, Column, Container, Length, PickList, Row, Text};

use crate::messages::Message;
use crate::params::SoundParameter;
use crate::style;

pub fn arp_grid_list<'a>(
    label: &'a str,
    state: &'a mut pick_list::State<ArpGrid>,
    sound_param: SoundParameter,
    value: i32,
) -> Container<'a, Message> {
    let value = match value {
        0 => Some(ArpGrid::Div48),
        1 => Some(ArpGrid::Div32),
        2 => Some(ArpGrid::Div24),
        3 => Some(ArpGrid::Div16),
        4 => Some(ArpGrid::Div12),
        5 => Some(ArpGrid::Div8),
        6 => Some(ArpGrid::Div6),
        7 => Some(ArpGrid::Div3),
        8 => Some(ArpGrid::Div2),
        9 => Some(ArpGrid::Div1),
        _ => None,
    };
    let pick_list = PickList::new(state, &ArpGrid::ALL[..], value, move |v| {
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
    Div3,
    Div2,
    Div1,
}

impl ArpGrid {
    const ALL: [ArpGrid; 10] = [
        ArpGrid::Div48,
        ArpGrid::Div32,
        ArpGrid::Div24,
        ArpGrid::Div16,
        ArpGrid::Div12,
        ArpGrid::Div8,
        ArpGrid::Div6,
        ArpGrid::Div3,
        ArpGrid::Div2,
        ArpGrid::Div1,
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
                ArpGrid::Div3 => "1/32",
                ArpGrid::Div2 => "1/32 triplet",
                ArpGrid::Div1 => "1/64 triplet",
            }
        )
    }
}
