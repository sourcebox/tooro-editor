//! Dropdown menu for the wavetables

use iced::widget::{Column, Container, PickList, Row, Text};

use crate::messages::Message;
use crate::params::SoundParameter;
use crate::style;

pub fn wavetable_list(label: &str, sound_param: SoundParameter, value: i32) -> Container<Message> {
    let value = match value {
        0 => Some(Wavetable::Factory1),
        1 => Some(Wavetable::Factory2),
        2 => Some(Wavetable::Factory3),
        3 => Some(Wavetable::Factory4),
        4 => Some(Wavetable::Factory5),
        5 => Some(Wavetable::Factory6),
        6 => Some(Wavetable::Factory7),
        7 => Some(Wavetable::Factory8),
        8 => Some(Wavetable::User),
        9 => Some(Wavetable::RandomNote),
        10 => Some(Wavetable::RandomChord),
        _ => None,
    };
    let pick_list = PickList::new(&Wavetable::ALL[..], value, move |v| {
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
                            .width(style::PARAM_LABEL_WIDTH),
                    )
                    .padding([4, 0, 0, 0]),
            )
            .push(pick_list),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wavetable {
    Factory1,
    Factory2,
    Factory3,
    Factory4,
    Factory5,
    Factory6,
    Factory7,
    Factory8,
    User,
    RandomNote,
    RandomChord,
}

impl Wavetable {
    const ALL: [Wavetable; 11] = [
        Wavetable::Factory1,
        Wavetable::Factory2,
        Wavetable::Factory3,
        Wavetable::Factory4,
        Wavetable::Factory5,
        Wavetable::Factory6,
        Wavetable::Factory7,
        Wavetable::Factory8,
        Wavetable::User,
        Wavetable::RandomNote,
        Wavetable::RandomChord,
    ];
}

impl std::fmt::Display for Wavetable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Wavetable::Factory1 => "Basic",
                Wavetable::Factory2 => "Bright",
                Wavetable::Factory3 => "Full",
                Wavetable::Factory4 => "Formants",
                Wavetable::Factory5 => "Mix 1",
                Wavetable::Factory6 => "Mix 2",
                Wavetable::Factory7 => "Mix 3",
                Wavetable::Factory8 => "Mix 4",
                Wavetable::User => "User",
                Wavetable::RandomNote => "Random Note",
                Wavetable::RandomChord => "Random Chord",
            }
        )
    }
}
