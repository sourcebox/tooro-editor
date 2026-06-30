//! Section containing the extra parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues, SoundParameter};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct ExtraSection;

impl ExtraSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &ParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Extra").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Noise",
                    Parameter::Sound(SoundParameter::ExtraNoise),
                    params.get_value(Parameter::Sound(SoundParameter::ExtraNoise)),
                ),
                slider_with_labels(
                    "O1xO2",
                    Parameter::Sound(SoundParameter::ExtraRingMod),
                    params.get_value(Parameter::Sound(SoundParameter::ExtraRingMod)),
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xF9, 0xB0, 0x8B)))
        .into()
    }
}
