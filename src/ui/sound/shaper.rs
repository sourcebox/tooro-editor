//! Section containing the shaper parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{shaper_mode_list::shaper_mode_list, slider::slider_with_labels};
use crate::ui::style;

pub struct ShaperSection {}

impl ShaperSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Shaper").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Cutoff",
                    SoundParameter::ShaperCutoff,
                    params.get_value(SoundParameter::ShaperCutoff),
                ),
                slider_with_labels(
                    "Resonance",
                    SoundParameter::ShaperResonance,
                    params.get_value(SoundParameter::ShaperResonance),
                ),
                slider_with_labels(
                    "Env A Amt",
                    SoundParameter::ShaperEnvAAmount,
                    params.get_value(SoundParameter::ShaperEnvAAmount),
                ),
                slider_with_labels(
                    "Track",
                    SoundParameter::ShaperTrack,
                    params.get_value(SoundParameter::ShaperTrack),
                ),
                shaper_mode_list(
                    "Mode",
                    SoundParameter::ShaperMode,
                    params.get_value(SoundParameter::ShaperMode),
                ),
                slider_with_labels(
                    "LFO 2 Amt",
                    SoundParameter::ShaperLFO2Amount,
                    params.get_value(SoundParameter::ShaperLFO2Amount),
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
