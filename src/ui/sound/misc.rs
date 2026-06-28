//! Section containing misc parameters

use iced::{
    Color, Element, Length, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{checkbox::checkbox_with_labels, slider::slider_with_labels};
use crate::ui::style;

pub struct MiscSection;

impl MiscSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Misc").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Bend Amt",
                    SoundParameter::BendRange,
                    params.get_value(SoundParameter::BendRange),
                ),
                slider_with_labels(
                    "Tune",
                    SoundParameter::Tune,
                    params.get_value(SoundParameter::Tune),
                ),
                checkbox_with_labels(
                    "",
                    "Poly",
                    SoundParameter::PolyMode,
                    params.get_value(SoundParameter::PolyMode),
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
