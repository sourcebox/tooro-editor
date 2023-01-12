//! Section containing the amplifier parameters

use iced::widget::{Column, Container, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct AmpSection {}

impl AmpSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Amp").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Level",
                SoundParameter::AmpLevel,
                params.get_value(SoundParameter::AmpLevel),
            ))
            .push(slider_with_labels(
                "Pan",
                SoundParameter::AmpPan,
                params.get_value(SoundParameter::AmpPan),
            ));
        Container::new(content).style(style::AmpSection).into()
    }
}
