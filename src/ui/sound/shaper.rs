//! Section containing the shaper parameters

use iced::widget::{Column, Container, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{shaper_mode_list::shaper_mode_list, slider::slider_with_labels};
use crate::ui::style;

pub struct ShaperSection {}

impl ShaperSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Shaper").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Cutoff",
                SoundParameter::ShaperCutoff,
                params.get_value(SoundParameter::ShaperCutoff),
            ))
            .push(slider_with_labels(
                "Resonance",
                SoundParameter::ShaperResonance,
                params.get_value(SoundParameter::ShaperResonance),
            ))
            .push(slider_with_labels(
                "Env A Amt",
                SoundParameter::ShaperEnvAAmount,
                params.get_value(SoundParameter::ShaperEnvAAmount),
            ))
            .push(slider_with_labels(
                "Track",
                SoundParameter::ShaperTrack,
                params.get_value(SoundParameter::ShaperTrack),
            ))
            .push(shaper_mode_list(
                "Mode",
                SoundParameter::ShaperMode,
                params.get_value(SoundParameter::ShaperMode),
            ))
            .push(slider_with_labels(
                "LFO 2 Amt",
                SoundParameter::ShaperLFO2Amount,
                params.get_value(SoundParameter::ShaperLFO2Amount),
            ));
        Container::new(content)
            // .style(style::ShaperSection)
            .into()
    }
}
