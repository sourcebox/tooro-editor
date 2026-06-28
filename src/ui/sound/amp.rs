//! Section containing the amplifier parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct AmpSection;

impl AmpSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Amp").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Level",
                    SoundParameter::AmpLevel,
                    params.get_value(SoundParameter::AmpLevel),
                ),
                slider_with_labels(
                    "Pan",
                    SoundParameter::AmpPan,
                    params.get_value(SoundParameter::AmpPan),
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0x65, 0xA4, 0x7E)))
        .into()
    }
}
