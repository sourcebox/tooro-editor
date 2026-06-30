//! Panel containing the multi parameters

mod fx;
mod midi;
mod mixer;

use iced::widget::{column, container};
use iced::{Element, Length};

use crate::messages::Message;
use crate::params::ParameterValues;
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

    pub fn view(&self, params: &ParameterValues) -> Element<'_, Message> {
        container(
            column![
                self.midi_section.view(params),
                self.mixer_section.view(params),
                self.fx_section.view(params)
            ]
            .spacing(10),
        )
        .height(Length::Fill)
        .into()
    }
}
