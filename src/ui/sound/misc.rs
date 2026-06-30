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

    pub fn view<'a>(&'a self, values: &'a ParameterValues) -> Element<'a, Message> {
        container(
            column![
                text("Misc").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Bend Amt",
                    Parameter::Sound(SoundParameter::BendRange),
                    values
                ),
                slider_with_labels("Tune", Parameter::Sound(SoundParameter::Tune), values),
                checkbox_with_labels(
                    "",
                    "Poly",
                    Parameter::Sound(SoundParameter::PolyMode),
                    values
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
