//! Section containing the MIDI settings

use iced::widget::{Column, Container, Text};
use iced::{Element, Length};

use crate::messages::Message;
use crate::params::{GetValue, MultiParameter, MultiParameterValues};
use crate::ui::elements::midi_channel_list::midi_channel_list;
use crate::ui::style;

pub struct MidiSection {}

impl MidiSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &MultiParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("MIDI").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(midi_channel_list(
                "Part 1 Ch",
                MultiParameter::ChannelPart1,
                params.get_value(MultiParameter::ChannelPart1),
            ))
            .push(midi_channel_list(
                "Part 2 Ch",
                MultiParameter::ChannelPart2,
                params.get_value(MultiParameter::ChannelPart2),
            ))
            .push(midi_channel_list(
                "Part 3 Ch",
                MultiParameter::ChannelPart3,
                params.get_value(MultiParameter::ChannelPart3),
            ))
            .push(midi_channel_list(
                "Part 4 Ch",
                MultiParameter::ChannelPart4,
                params.get_value(MultiParameter::ChannelPart4),
            ));
        Container::new(content)
            // .style(style::MidiSection)
            .width(Length::Fill)
            .into()
    }
}
