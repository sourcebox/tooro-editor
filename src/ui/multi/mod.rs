//! User interface panel containing the multi parameters

mod fx;
mod midi;
mod mixer;

use iced::{Column, Container, Element, Length, Row};

use super::style;
use crate::messages::Message;
use crate::params::MultiParameterValues;
use fx::FXSection;
use midi::MidiSection;
use mixer::MixerSection;

pub struct MultiPanel {
    midi_section: MidiSection,
    mixer_section: MixerSection,
    fx_section: FXSection,
}

impl MultiPanel {
    pub fn new() -> Self {
        Self {
            midi_section: MidiSection::new(),
            mixer_section: MixerSection::new(),
            fx_section: FXSection::new(),
        }
    }

    pub fn view(&mut self, params: &MultiParameterValues) -> Element<Message> {
        let col = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.midi_section.view(params))
            .push(self.mixer_section.view(params))
            .push(self.fx_section.view(params));

        Container::new(Column::new().push(Row::new().push(col)))
            .padding(5)
            .height(Length::Fill)
            .style(style::MainWindow)
            .into()
    }
}
