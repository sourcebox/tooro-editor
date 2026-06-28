//! Slider control wrapped in a container with label and value display

use iced::{
    Padding, alignment,
    widget::{Container, container, row, text},
};

use super::super::widgets::slider::Slider;

use crate::messages::Message;
use crate::params::{MultiParameter, SoundParameter};
use crate::style;

/// Returns a slider for a sound (preset) parameter.
pub fn slider_with_labels(
    label: &str,
    sound_param: SoundParameter,
    value: i32,
) -> Container<'_, Message> {
    let range = sound_param.get_range();
    let slider = Slider::new(range, value, move |v| {
        Message::SoundParameterChange(sound_param, v)
    })
    .default(sound_param.get_default())
    .style(|_, status| style::slider(status))
    .shift_step(0.25);

    container(row![
        container(
            text(label)
                .size(style::PARAM_LABEL_TEXT_SIZE)
                .width(style::PARAM_LABEL_WIDTH),
        ),
        slider,
        container(
            text(format!("{}", value))
                .size(style::PARAM_LABEL_TEXT_SIZE)
                .align_x(alignment::Horizontal::Right)
                .width(style::PARAM_VALUE_WIDTH)
        )
    ])
}

/// Returns a slider for a multi parameter.
pub fn multi_slider_with_labels(
    label: &str,
    multi_param: MultiParameter,
    value: i32,
) -> Container<'_, Message> {
    let range = multi_param.get_range();
    let slider = Slider::new(range, value, move |v| {
        Message::MultiParameterChange(multi_param, v)
    })
    .default(multi_param.get_default())
    .style(|_, status| style::slider(status))
    .shift_step(0.25);

    container(row![
        container(
            text(label)
                .size(style::PARAM_LABEL_TEXT_SIZE)
                .width(style::PARAM_LABEL_WIDTH),
        ),
        slider,
        container(
            text(format!("{}", value))
                .size(style::PARAM_LABEL_TEXT_SIZE)
                .align_x(alignment::Horizontal::Right)
                .width(style::PARAM_VALUE_WIDTH)
        )
    ])
}
