//! Section containing the arpeggiator parameters

use iced::{
    Color, Element, Length, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues, SoundParameter};
use crate::ui::elements::{
    arp_grid_list::arp_grid_list, arp_mode_list::arp_mode_list, checkbox::checkbox_with_labels,
    slider::slider_with_labels,
};
use crate::ui::style;

pub struct ArpSection;

impl ArpSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&'a self, values: &'a ParameterValues) -> Element<'a, Message> {
        container(
            column![
                text("Arp").size(style::SECTION_LABEL_TEXT_SIZE),
                arp_mode_list("Mode", Parameter::Sound(SoundParameter::ArpMode), values),
                arp_grid_list("Grid", Parameter::Sound(SoundParameter::ArpGrid), values),
                slider_with_labels("Tempo", Parameter::Sound(SoundParameter::ArpTempo), values),
                checkbox_with_labels(
                    "",
                    "Hold",
                    Parameter::Sound(SoundParameter::ArpHold),
                    values
                )
            ]
            .height(Length::Fill)
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xF9, 0xB0, 0x8B)))
        .into()
    }
}
