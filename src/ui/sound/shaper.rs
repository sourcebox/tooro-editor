//! Section containing the shaper parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues, SoundParameter};
use crate::ui::elements::{shaper_mode_list::shaper_mode_list, slider::slider_with_labels};
use crate::ui::style;

pub struct ShaperSection {}

impl ShaperSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&'a self, values: &'a ParameterValues) -> Element<'a, Message> {
        container(
            column![
                text("Shaper").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Cutoff",
                    Parameter::Sound(SoundParameter::ShaperCutoff),
                    values
                ),
                slider_with_labels(
                    "Resonance",
                    Parameter::Sound(SoundParameter::ShaperResonance),
                    values
                ),
                slider_with_labels(
                    "Env A Amt",
                    Parameter::Sound(SoundParameter::ShaperEnvAAmount),
                    values
                ),
                slider_with_labels(
                    "Track",
                    Parameter::Sound(SoundParameter::ShaperTrack),
                    values
                ),
                shaper_mode_list("Mode", Parameter::Sound(SoundParameter::ShaperMode), values),
                slider_with_labels(
                    "LFO 2 Amt",
                    Parameter::Sound(SoundParameter::ShaperLFO2Amount),
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
