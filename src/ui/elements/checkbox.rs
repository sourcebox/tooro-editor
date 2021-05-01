use iced::{Checkbox, Container, Length, Row, Text};

use crate::messages::Message;
use crate::params::SoundParameter;
use crate::style;

pub fn checkbox_with_labels<'a>(
    label: &'a str,
    text: &'a str,
    sound_param: SoundParameter,
    value: i32,
) -> Container<'a, Message> {
    let checkbox = Checkbox::new(value != 0, text, move |v| {
        Message::SoundParameterChange(sound_param, v as i32)
    })
    .style(style::Checkbox)
    .text_size(style::LIST_ITEM_TEXT_SIZE);

    Container::new(
        Row::new()
            .push(
                Text::new(label)
                    .size(style::PARAM_LABEL_TEXT_SIZE)
                    .width(Length::Units(80)),
            )
            .push(checkbox),
    )
}
