//! Slider control wrapped in a container with label and value display

use iced::alignment;
use iced::widget::{Column, Container, Row, Text};

use super::slider_widget::Slider;

use crate::messages::Message;
use crate::params::{MultiParameter, SoundParameter};
use crate::style;

/// Returns a slider for a sound (preset) parameter
pub fn slider_with_labels(
    label: &str,
    sound_param: SoundParameter,
    value: i32,
) -> Container<Message> {
    let range = sound_param.get_range();
    let slider = Slider::new(range, value, sound_param.get_default(), move |v| {
        Message::SoundParameterChange(sound_param, v)
    })
    .style(style::Slider);

    Container::new(
        Row::new()
            .push(
                Column::new()
                    .push(
                        Text::new(label)
                            .size(style::PARAM_LABEL_TEXT_SIZE)
                            .width(style::PARAM_LABEL_WIDTH),
                    )
                    .padding([3, 0, 0, 0]),
            )
            .push(slider)
            .push(
                Column::new()
                    .push(
                        Text::new(format!("{}", value))
                            .size(style::PARAM_LABEL_TEXT_SIZE)
                            .horizontal_alignment(alignment::Horizontal::Right)
                            .width(style::PARAM_VALUE_WIDTH),
                    )
                    .padding([3, 0, 0, 5]),
            ),
    )
}

/// Returns a slider for a multi parameter
pub fn multi_slider_with_labels(
    label: &str,
    multi_param: MultiParameter,
    value: i32,
) -> Container<Message> {
    let range = multi_param.get_range();
    let slider = Slider::new(range, value, multi_param.get_default(), move |v| {
        Message::MultiParameterChange(multi_param, v)
    })
    .style(style::Slider);

    Container::new(
        Row::new()
            .push(
                Column::new()
                    .push(
                        Text::new(label)
                            .size(style::PARAM_LABEL_TEXT_SIZE)
                            .width(style::PARAM_LABEL_WIDTH),
                    )
                    .padding([3, 0, 0, 0]),
            )
            .push(slider)
            .push(
                Column::new()
                    .push(
                        Text::new(format!("{}", value))
                            .size(style::PARAM_LABEL_TEXT_SIZE)
                            .horizontal_alignment(alignment::Horizontal::Right)
                            .width(style::PARAM_VALUE_WIDTH),
                    )
                    .padding([3, 0, 0, 5]),
            ),
    )
}
