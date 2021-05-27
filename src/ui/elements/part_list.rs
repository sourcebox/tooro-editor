//! Dropdown menu for part selection

use iced::{pick_list, Container, PickList};

use crate::messages::Message;
use crate::style;

pub fn part_list<'a>(
    state: &'a mut pick_list::State<PartList>,
    value: u8,
) -> Container<'a, Message> {
    let value = match value {
        0 => Some(PartList::Part1),
        1 => Some(PartList::Part2),
        2 => Some(PartList::Part3),
        3 => Some(PartList::Part4),
        _ => None,
    };
    let pick_list = PickList::new(state, &PartList::ALL[..], value, move |v| {
        Message::PartChange(v as u8)
    })
    .style(style::PickList)
    .text_size(style::LIST_ITEM_TEXT_SIZE);

    Container::new(pick_list)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartList {
    Part1,
    Part2,
    Part3,
    Part4,
}

impl PartList {
    const ALL: [PartList; 4] = [
        PartList::Part1,
        PartList::Part2,
        PartList::Part3,
        PartList::Part4,
    ];
}

impl std::fmt::Display for PartList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PartList::Part1 => "Part 1",
                PartList::Part2 => "Part 2",
                PartList::Part3 => "Part 3",
                PartList::Part4 => "Part 4",
            }
        )
    }
}
