//! Section containing oscillator 2 parameters

use iced::widget::{Column, Container, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{slider::slider_with_labels, wavetable_list::wavetable_list};
use crate::ui::style;

pub struct Osc2Section {}

impl Osc2Section {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Osc 2").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(wavetable_list(
                "Table",
                SoundParameter::Osc2Table,
                params.get_value(SoundParameter::Osc2Table),
            ))
            .push(slider_with_labels(
                "Wave",
                SoundParameter::Osc2Wave,
                params.get_value(SoundParameter::Osc2Wave),
            ))
            .push(slider_with_labels(
                "Coarse",
                SoundParameter::Osc2Coarse,
                params.get_value(SoundParameter::Osc2Coarse),
            ))
            .push(slider_with_labels(
                "Fine",
                SoundParameter::Osc2Fine,
                params.get_value(SoundParameter::Osc2Fine),
            ))
            .push(slider_with_labels(
                "FM Amt",
                SoundParameter::Osc2FMAmount,
                params.get_value(SoundParameter::Osc2FMAmount),
            ))
            .push(slider_with_labels(
                "FM Rate",
                SoundParameter::Osc2FMRate,
                params.get_value(SoundParameter::Osc2FMRate),
            ))
            .push(slider_with_labels(
                "Sync",
                SoundParameter::Osc2Sync,
                params.get_value(SoundParameter::Osc2Sync),
            ))
            .push(slider_with_labels(
                "Level",
                SoundParameter::Osc2Level,
                params.get_value(SoundParameter::Osc2Level),
            ));
        Container::new(content)
            // .style(style::OscSection)
            .into()
    }
}
