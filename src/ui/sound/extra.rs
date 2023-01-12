//! Section containing the extra parameters

use iced::widget::{Column, Container, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct ExtraSection {}

impl ExtraSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Extra").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Noise",
                SoundParameter::ExtraNoise,
                params.get_value(SoundParameter::ExtraNoise),
            ))
            .push(slider_with_labels(
                "O1xO2",
                SoundParameter::ExtraRingMod,
                params.get_value(SoundParameter::ExtraRingMod),
            ));
        Container::new(content)
            // .style(style::ExtraSection)
            .into()
    }
}
