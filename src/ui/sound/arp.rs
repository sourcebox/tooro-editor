use iced::{pick_list, slider, Column, Container, Element, Text};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{
    arp_grid_list::{arp_grid_list, ArpGrid},
    arp_mode_list::{arp_mode_list, ArpMode},
    checkbox::checkbox_with_labels,
    slider::slider_with_labels,
};
use crate::ui::style;

pub struct ArpSection {
    mode_list: pick_list::State<ArpMode>,
    grid_list: pick_list::State<ArpGrid>,
    tempo_slider: slider::State,
}

impl ArpSection {
    pub fn new() -> Self {
        Self {
            mode_list: pick_list::State::<ArpMode>::default(),
            grid_list: pick_list::State::<ArpGrid>::default(),
            tempo_slider: slider::State::new(),
        }
    }

    pub fn view(&mut self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Arp").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(arp_mode_list(
                "Mode",
                &mut self.mode_list,
                SoundParameter::ArpMode,
                params.get_value(SoundParameter::ArpMode),
            ))
            .push(arp_grid_list(
                "Grid",
                &mut self.grid_list,
                SoundParameter::ArpGrid,
                params.get_value(SoundParameter::ArpGrid),
            ))
            .push(slider_with_labels(
                "Tempo",
                &mut self.tempo_slider,
                SoundParameter::ArpTempo,
                params.get_value(SoundParameter::ArpTempo),
            ))
            .push(checkbox_with_labels(
                "",
                "Hold",
                SoundParameter::ArpHold,
                params.get_value(SoundParameter::ArpHold),
            ));
        Container::new(content).style(style::ArpSection).into()
    }
}
