//! Section containing oscillator 1 parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{slider::slider_with_labels, wavetable_list::wavetable_list};
use crate::ui::style;

pub struct Osc1Section;

impl Osc1Section {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Osc 1").size(style::SECTION_LABEL_TEXT_SIZE),
                wavetable_list(
                    "Table",
                    SoundParameter::Osc1Table,
                    params.get_value(SoundParameter::Osc1Table),
                ),
                slider_with_labels(
                    "Wave",
                    SoundParameter::Osc1Wave,
                    params.get_value(SoundParameter::Osc1Wave),
                ),
                slider_with_labels(
                    "Coarse",
                    SoundParameter::Osc1Coarse,
                    params.get_value(SoundParameter::Osc1Coarse),
                ),
                slider_with_labels(
                    "Fine",
                    SoundParameter::Osc1Fine,
                    params.get_value(SoundParameter::Osc1Fine),
                ),
                slider_with_labels(
                    "FM Amt",
                    SoundParameter::Osc1FMAmount,
                    params.get_value(SoundParameter::Osc1FMAmount),
                ),
                slider_with_labels(
                    "FM Rate",
                    SoundParameter::Osc1FMRate,
                    params.get_value(SoundParameter::Osc1FMRate),
                ),
                slider_with_labels(
                    "Sync",
                    SoundParameter::Osc1Sync,
                    params.get_value(SoundParameter::Osc1Sync),
                ),
                slider_with_labels(
                    "Level",
                    SoundParameter::Osc1Level,
                    params.get_value(SoundParameter::Osc1Level),
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xAB, 0xA3, 0x39)))
        .into()
    }
}
