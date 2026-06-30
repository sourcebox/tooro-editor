//! Section containing the MIDI settings

use iced::{
    Color, Element, Length, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{MultiParameter, Parameter, ParameterValues};
use crate::ui::elements::midi_channel_list::midi_channel_list;
use crate::ui::style;

pub struct MidiSection;

impl MidiSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&'a self, values: &'a ParameterValues) -> Element<'a, Message> {
        container(
            column![
                text("MIDI").size(style::SECTION_LABEL_TEXT_SIZE),
                midi_channel_list(
                    "Part 1 Ch",
                    Parameter::Multi(MultiParameter::ChannelPart1),
                    values
                ),
                midi_channel_list(
                    "Part 2 Ch",
                    Parameter::Multi(MultiParameter::ChannelPart2),
                    values
                ),
                midi_channel_list(
                    "Part 3 Ch",
                    Parameter::Multi(MultiParameter::ChannelPart3),
                    values
                ),
                midi_channel_list(
                    "Part 4 Ch",
                    Parameter::Multi(MultiParameter::ChannelPart4),
                    values
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .width(Length::Fill)
        .style(|_| style::section(Color::from_rgb8(0xC0, 0xC0, 0xC0)))
        .into()
    }
}
