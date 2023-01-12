//! Section containing the filter parameters

use iced::widget::{Column, Container, Text};
use iced::{Element, Length};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct FilterSection {}

impl FilterSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Filter").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .height(Length::Units(169))
            .push(slider_with_labels(
                "Cutoff",
                SoundParameter::FilterCutoff,
                params.get_value(SoundParameter::FilterCutoff),
            ))
            .push(slider_with_labels(
                "Resonance",
                SoundParameter::FilterResonance,
                params.get_value(SoundParameter::FilterResonance),
            ))
            .push(slider_with_labels(
                "Env F Amt",
                SoundParameter::FilterEnvFAmount,
                params.get_value(SoundParameter::FilterEnvFAmount),
            ))
            .push(slider_with_labels(
                "Track",
                SoundParameter::FilterTrack,
                params.get_value(SoundParameter::FilterTrack),
            ))
            .push(slider_with_labels(
                "After",
                SoundParameter::FilterAfter,
                params.get_value(SoundParameter::FilterAfter),
            ))
            .push(slider_with_labels(
                "LFO 1 Amt",
                SoundParameter::FilterLFO1Amount,
                params.get_value(SoundParameter::FilterLFO1Amount),
            ));
        Container::new(content).style(style::FilterSection).into()
    }
}
