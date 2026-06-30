//! Section containing the filter parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues, SoundParameter};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct FilterSection;

impl FilterSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&'a self, values: &'a ParameterValues) -> Element<'a, Message> {
        container(
            column![
                text("Filter").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Cutoff",
                    Parameter::Sound(SoundParameter::FilterCutoff),
                    values
                ),
                slider_with_labels(
                    "Resonance",
                    Parameter::Sound(SoundParameter::FilterResonance),
                    values
                ),
                slider_with_labels(
                    "Env F Amt",
                    Parameter::Sound(SoundParameter::FilterEnvFAmount),
                    values
                ),
                slider_with_labels(
                    "Track",
                    Parameter::Sound(SoundParameter::FilterTrack),
                    values
                ),
                slider_with_labels(
                    "After",
                    Parameter::Sound(SoundParameter::FilterAfter),
                    values
                ),
                slider_with_labels(
                    "LFO 1 Amt",
                    Parameter::Sound(SoundParameter::FilterLFO1Amount),
                    values
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING)
            .height(170),
        )
        .style(|_| style::section(Color::from_rgb8(0xD8, 0x00, 0x00)))
        .into()
    }
}
