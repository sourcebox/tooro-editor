//! Section containing the arpeggiator parameters

use iced::widget::{Column, Container, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{
    arp_grid_list::arp_grid_list, arp_mode_list::arp_mode_list, checkbox::checkbox_with_labels,
    slider::slider_with_labels,
};
use crate::ui::style;

pub struct ArpSection {}

impl ArpSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Arp").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(arp_mode_list(
                "Mode",
                SoundParameter::ArpMode,
                params.get_value(SoundParameter::ArpMode),
            ))
            .push(arp_grid_list(
                "Grid",
                SoundParameter::ArpGrid,
                params.get_value(SoundParameter::ArpGrid),
            ))
            .push(slider_with_labels(
                "Tempo",
                SoundParameter::ArpTempo,
                params.get_value(SoundParameter::ArpTempo),
            ))
            .push(checkbox_with_labels(
                "",
                "Hold",
                SoundParameter::ArpHold,
                params.get_value(SoundParameter::ArpHold),
            ));
        Container::new(content)
            // .style(style::ArpSection)
            .into()
    }
}
