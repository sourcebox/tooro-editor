//! Slider widget wrapped in a container with label and value display.

use iced::{
    alignment,
    widget::{Container, container, row, text},
};

use super::super::widgets::slider::Slider;

use crate::messages::Message;
use crate::params::Parameter;
use crate::style;

/// Returns a slider for a parameter.
pub fn slider_with_labels(label: &str, param: Parameter, value: i32) -> Container<'_, Message> {
    let range = param.get_range();
    let slider = Slider::new(range, value, move |v| Message::ParameterChange(param, v))
        .default(param.get_default())
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
            text(value)
                .size(style::PARAM_LABEL_TEXT_SIZE)
                .align_x(alignment::Horizontal::Right)
                .width(style::PARAM_VALUE_WIDTH)
        )
    ])
}
