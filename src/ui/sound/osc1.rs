//! Section containing oscillator 1 parameters

use iced::widget::{Column, Container, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{slider::slider_with_labels, wavetable_list::wavetable_list};
use crate::ui::style;

pub struct Osc1Section {}

impl Osc1Section {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Osc 1").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(wavetable_list(
                "Table",
                SoundParameter::Osc1Table,
                params.get_value(SoundParameter::Osc1Table),
            ))
            .push(slider_with_labels(
                "Wave",
                SoundParameter::Osc1Wave,
                params.get_value(SoundParameter::Osc1Wave),
            ))
            .push(slider_with_labels(
                "Coarse",
                SoundParameter::Osc1Coarse,
                params.get_value(SoundParameter::Osc1Coarse),
            ))
            .push(slider_with_labels(
                "Fine",
                SoundParameter::Osc1Fine,
                params.get_value(SoundParameter::Osc1Fine),
            ))
            .push(slider_with_labels(
                "FM Amt",
                SoundParameter::Osc1FMAmount,
                params.get_value(SoundParameter::Osc1FMAmount),
            ))
            .push(slider_with_labels(
                "FM Rate",
                SoundParameter::Osc1FMRate,
                params.get_value(SoundParameter::Osc1FMRate),
            ))
            .push(slider_with_labels(
                "Sync",
                SoundParameter::Osc1Sync,
                params.get_value(SoundParameter::Osc1Sync),
            ))
            .push(slider_with_labels(
                "Level",
                SoundParameter::Osc1Level,
                params.get_value(SoundParameter::Osc1Level),
            ));
        Container::new(content)
            // .style(style::Container::OscSection)
            .into()
    }
}
