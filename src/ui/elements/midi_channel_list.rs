//! Dropdown menu for the MIDI channels

use iced::{pick_list, Column, Container, Length, PickList, Row, Text};

use crate::messages::Message;
use crate::params::MultiParameter;
use crate::style;

pub fn midi_channel_list<'a>(
    label: &'a str,
    state: &'a mut pick_list::State<MidiChannel>,
    multi_param: MultiParameter,
    value: i32,
) -> Container<'a, Message> {
    let value = match value {
        0 => Some(MidiChannel::Omni),
        1 => Some(MidiChannel::Channel1),
        2 => Some(MidiChannel::Channel2),
        3 => Some(MidiChannel::Channel3),
        4 => Some(MidiChannel::Channel4),
        5 => Some(MidiChannel::Channel5),
        6 => Some(MidiChannel::Channel6),
        7 => Some(MidiChannel::Channel7),
        8 => Some(MidiChannel::Channel8),
        9 => Some(MidiChannel::Channel9),
        10 => Some(MidiChannel::Channel10),
        11 => Some(MidiChannel::Channel11),
        12 => Some(MidiChannel::Channel12),
        13 => Some(MidiChannel::Channel13),
        14 => Some(MidiChannel::Channel14),
        15 => Some(MidiChannel::Channel15),
        16 => Some(MidiChannel::Channel16),
        _ => None,
    };
    let pick_list = PickList::new(state, &MidiChannel::ALL[..], value, move |v| {
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
pub enum MidiChannel {
    Omni,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8,
    Channel9,
    Channel10,
    Channel11,
    Channel12,
    Channel13,
    Channel14,
    Channel15,
    Channel16,
}

impl MidiChannel {
    const ALL: [MidiChannel; 17] = [
        MidiChannel::Omni,
        MidiChannel::Channel1,
        MidiChannel::Channel2,
        MidiChannel::Channel3,
        MidiChannel::Channel4,
        MidiChannel::Channel5,
        MidiChannel::Channel6,
        MidiChannel::Channel7,
        MidiChannel::Channel8,
        MidiChannel::Channel9,
        MidiChannel::Channel10,
        MidiChannel::Channel11,
        MidiChannel::Channel12,
        MidiChannel::Channel13,
        MidiChannel::Channel14,
        MidiChannel::Channel15,
        MidiChannel::Channel16,
    ];
}

impl std::fmt::Display for MidiChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MidiChannel::Omni => "Omni",
                MidiChannel::Channel1 => "Channel 1",
                MidiChannel::Channel2 => "Channel 2",
                MidiChannel::Channel3 => "Channel 3",
                MidiChannel::Channel4 => "Channel 4",
                MidiChannel::Channel5 => "Channel 5",
                MidiChannel::Channel6 => "Channel 6",
                MidiChannel::Channel7 => "Channel 7",
                MidiChannel::Channel8 => "Channel 8",
                MidiChannel::Channel9 => "Channel 9",
                MidiChannel::Channel10 => "Channel 10",
                MidiChannel::Channel11 => "Channel 11",
                MidiChannel::Channel12 => "Channel 12",
                MidiChannel::Channel13 => "Channel 13",
                MidiChannel::Channel14 => "Channel 14",
                MidiChannel::Channel15 => "Channel 15",
                MidiChannel::Channel16 => "Channel 16",
            }
        )
    }
}
