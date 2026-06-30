//! Section containing misc parameters

use iced::{
    Color, Element, Length, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues, SoundParameter};
use crate::ui::elements::{checkbox::checkbox_with_labels, slider::slider_with_labels};
use crate::ui::style;

pub struct MiscSection;

impl MiscSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &ParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Misc").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Bend Amt",
                    Parameter::Sound(SoundParameter::BendRange),
                    params.get_value(Parameter::Sound(SoundParameter::BendRange)),
                ),
                slider_with_labels(
                    "Tune",
                    Parameter::Sound(SoundParameter::Tune),
                    params.get_value(Parameter::Sound(SoundParameter::Tune)),
                ),
                checkbox_with_labels(
                    "",
                    "Poly",
                    Parameter::Sound(SoundParameter::PolyMode),
                    params.get_value(Parameter::Sound(SoundParameter::PolyMode)),
                )
            ]
            .height(Length::Fill)
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xC0, 0xC0, 0xC0)))
        .into()
    }
}
