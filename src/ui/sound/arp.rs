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

    pub fn view(&self, params: &ParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Arp").size(style::SECTION_LABEL_TEXT_SIZE),
                arp_mode_list(
                    "Mode",
                    Parameter::Sound(SoundParameter::ArpMode),
                    params.get_value(Parameter::Sound(SoundParameter::ArpMode)),
                ),
                arp_grid_list(
                    "Grid",
                    Parameter::Sound(SoundParameter::ArpGrid),
                    params.get_value(Parameter::Sound(SoundParameter::ArpGrid)),
                ),
                slider_with_labels(
                    "Tempo",
                    Parameter::Sound(SoundParameter::ArpTempo),
                    params.get_value(Parameter::Sound(SoundParameter::ArpTempo)),
                ),
                checkbox_with_labels(
                    "",
                    "Hold",
                    Parameter::Sound(SoundParameter::ArpHold),
                    params.get_value(Parameter::Sound(SoundParameter::ArpHold)),
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
