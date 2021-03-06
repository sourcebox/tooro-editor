//! Slider control wrapped in a container with label and value display

use iced::{alignment, Column, Container, Length, Row, Text};

use super::slider_widget::{self, Slider};

use crate::messages::Message;
use crate::params::{MultiParameter, SoundParameter};
use crate::style;

/// Returns a slider for a sound (preset) parameter
pub fn slider_with_labels<'a>(
    label: &'a str,
    state: &'a mut slider_widget::State,
    sound_param: SoundParameter,
    value: i32,
) -> Container<'a, Message> {
    let range = sound_param.get_range();
    let slider = Slider::new(state, range, value, sound_param.get_default(), move |v| {
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
                            .width(Length::Units(style::PARAM_LABEL_WIDTH)),
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
                            .width(Length::Units(style::PARAM_VALUE_WIDTH)),
                    )
                    .padding([3, 0, 0, 5]),
            ),
    )
}

/// Returns a slider for a multi parameter
pub fn multi_slider_with_labels<'a>(
    label: &'a str,
    state: &'a mut slider_widget::State,
    multi_param: MultiParameter,
    value: i32,
) -> Container<'a, Message> {
    let range = multi_param.get_range();
    let slider = Slider::new(state, range, value, multi_param.get_default(), move |v| {
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
                            .width(Length::Units(style::PARAM_LABEL_WIDTH)),
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
                            .width(Length::Units(style::PARAM_VALUE_WIDTH)),
                    )
                    .padding([3, 0, 0, 5]),
            ),
    )
}
