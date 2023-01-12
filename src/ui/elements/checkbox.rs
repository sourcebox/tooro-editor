//! Checkbox control wrapped in a container with label

use iced::widget::{Checkbox, Container, Row, Text};
use iced::Length;

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
    .text_size(style::LIST_ITEM_TEXT_SIZE)
    .spacing(7);

    Container::new(
        Row::new()
            .push(
                Text::new(label)
                    .size(style::PARAM_LABEL_TEXT_SIZE)
                    .width(Length::Units(style::PARAM_LABEL_WIDTH)),
            )
            .push(checkbox),
    )
}
