//! Section containing the MIDI settings

use iced::{pick_list, Column, Container, Element, Length, Text};

use crate::messages::Message;
use crate::params::{GetValue, MultiParameter, MultiParameterValues};
use crate::ui::elements::midi_channel_list::{midi_channel_list, MidiChannel};
use crate::ui::style;

pub struct MidiSection {
    part1_ch_list: pick_list::State<MidiChannel>,
    part2_ch_list: pick_list::State<MidiChannel>,
    part3_ch_list: pick_list::State<MidiChannel>,
    part4_ch_list: pick_list::State<MidiChannel>,
}

impl MidiSection {
    pub fn new() -> Self {
        Self {
            part1_ch_list: pick_list::State::<MidiChannel>::default(),
            part2_ch_list: pick_list::State::<MidiChannel>::default(),
            part3_ch_list: pick_list::State::<MidiChannel>::default(),
            part4_ch_list: pick_list::State::<MidiChannel>::default(),
        }
    }

    pub fn view(&mut self, params: &MultiParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("MIDI").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(midi_channel_list(
                "Part 1 Ch",
                &mut self.part1_ch_list,
                MultiParameter::ChannelPart1,
                params.get_value(MultiParameter::ChannelPart1),
            ))
            .push(midi_channel_list(
                "Part 2 Ch",
                &mut self.part2_ch_list,
                MultiParameter::ChannelPart2,
                params.get_value(MultiParameter::ChannelPart2),
            ))
            .push(midi_channel_list(
                "Part 3 Ch",
                &mut self.part3_ch_list,
                MultiParameter::ChannelPart3,
                params.get_value(MultiParameter::ChannelPart3),
            ))
            .push(midi_channel_list(
                "Part 4 Ch",
                &mut self.part4_ch_list,
                MultiParameter::ChannelPart4,
                params.get_value(MultiParameter::ChannelPart4),
            ));
        Container::new(content)
            .style(style::MidiSection)
            .width(Length::Fill)
            .into()
    }
}
