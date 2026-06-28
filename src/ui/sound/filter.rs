//! Section containing the filter parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct FilterSection;

impl FilterSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Filter").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Cutoff",
                    SoundParameter::FilterCutoff,
                    params.get_value(SoundParameter::FilterCutoff),
                ),
                slider_with_labels(
                    "Resonance",
                    SoundParameter::FilterResonance,
                    params.get_value(SoundParameter::FilterResonance),
                ),
                slider_with_labels(
                    "Env F Amt",
                    SoundParameter::FilterEnvFAmount,
                    params.get_value(SoundParameter::FilterEnvFAmount),
                ),
                slider_with_labels(
                    "Track",
                    SoundParameter::FilterTrack,
                    params.get_value(SoundParameter::FilterTrack),
                ),
                slider_with_labels(
                    "After",
                    SoundParameter::FilterAfter,
                    params.get_value(SoundParameter::FilterAfter),
                ),
                slider_with_labels(
                    "LFO 1 Amt",
                    SoundParameter::FilterLFO1Amount,
                    params.get_value(SoundParameter::FilterLFO1Amount),
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING)
            .height(169),
        )
        .style(|_| style::section(Color::from_rgb8(0xD8, 0x00, 0x00)))
        .into()
    }
}
