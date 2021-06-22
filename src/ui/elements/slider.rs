//! Slider control wrapped in a container with label and value display

use iced::{slider, Column, Container, HorizontalAlignment, Length, Row, Slider, Text};

use crate::messages::Message;
use crate::params::{MultiParameter, SoundParameter};
use crate::style;

pub fn slider_with_labels<'a>(
    label: &'a str,
    state: &'a mut slider::State,
    sound_param: SoundParameter,
    value: i32,
) -> Container<'a, Message> {
    let range = sound_param.get_range();
    let slider = Slider::new(state, range, value, move |v| {
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
                            .horizontal_alignment(HorizontalAlignment::Right)
                            .width(Length::Units(style::PARAM_VALUE_WIDTH)),
                    )
                    .padding([3, 0, 0, 5]),
            ),
    )
}

pub fn multi_slider_with_labels<'a>(
    label: &'a str,
    state: &'a mut slider::State,
    multi_param: MultiParameter,
    value: i32,
) -> Container<'a, Message> {
    let range = multi_param.get_range();
    let slider = Slider::new(state, range, value, move |v| {
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
                            .horizontal_alignment(HorizontalAlignment::Right)
                            .width(Length::Units(style::PARAM_VALUE_WIDTH)),
                    )
                    .padding([3, 0, 0, 5]),
            ),
    )
}
