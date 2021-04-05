use iced::{slider, Container, HorizontalAlignment, Length, Row, Slider, Text};

use crate::messages::Message;
use crate::params::SoundParameter;
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
            .push(Text::new(label).size(16).width(Length::Units(80)))
            .push(slider)
            .push(
                Text::new(format!("{}", value))
                    .size(16)
                    .horizontal_alignment(HorizontalAlignment::Right)
                    .width(Length::Units(30)),
            ),
    )
}
