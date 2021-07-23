//! Section containing the extra parameters

use iced::{slider, Column, Container, Element, Text};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct ExtraSection {
    noise_slider: slider::State,
    ringmod_slider: slider::State,
}

impl ExtraSection {
    pub fn new() -> Self {
        Self {
            noise_slider: slider::State::new(),
            ringmod_slider: slider::State::new(),
        }
    }

    pub fn view(&mut self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Extra").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Noise",
                &mut self.noise_slider,
                SoundParameter::ExtraNoise,
                params.get_value(SoundParameter::ExtraNoise),
            ))
            .push(slider_with_labels(
                "RingMod",
                &mut self.ringmod_slider,
                SoundParameter::ExtraRingMod,
                params.get_value(SoundParameter::ExtraRingMod),
            ));
        Container::new(content).style(style::ExtraSection).into()
    }
}
