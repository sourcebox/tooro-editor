//! Checkbox control wrapped in a container with label.

use iced::widget::{Checkbox, Container, container, row, text};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues};
use crate::style;

pub fn checkbox_with_labels<'a>(
    label: &'a str,
    desc: &'a str,
    param: Parameter,
    values: &'a ParameterValues,
) -> Container<'a, Message> {
    let checkbox = Checkbox::new(values.get_value(param) != 0)
        .label(desc)
        .on_toggle(move |v| Message::ParameterChange(param, v as i32))
        .style(|_, status| style::checkbox(status))
        .text_size(style::LIST_ITEM_TEXT_SIZE)
        .spacing(7);

    container(row![
        text(label)
            .size(style::PARAM_LABEL_TEXT_SIZE)
            .width(style::PARAM_LABEL_WIDTH),
        checkbox
    ])
}
