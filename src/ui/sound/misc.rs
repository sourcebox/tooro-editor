//! Section containing misc parameters

use iced::widget::{Column, Container, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{checkbox::checkbox_with_labels, slider::slider_with_labels};
use crate::ui::style;

pub struct MiscSection {}

impl MiscSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Misc").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Bend Amt",
                SoundParameter::BendRange,
                params.get_value(SoundParameter::BendRange),
            ))
            .push(slider_with_labels(
                "Tune",
                SoundParameter::Tune,
                params.get_value(SoundParameter::Tune),
            ))
            .push(checkbox_with_labels(
                "",
                "Poly",
                SoundParameter::PolyMode,
                params.get_value(SoundParameter::PolyMode),
            ));
        Container::new(content).style(style::MiscSection).into()
    }
}
