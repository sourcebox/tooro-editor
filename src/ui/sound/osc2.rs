//! Section containing oscillator 2 parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{GetValue, Parameter, SoundParameter, SoundParameterValues};
use crate::ui::elements::{slider::slider_with_labels, wavetable_list::wavetable_list};
use crate::ui::style;

pub struct Osc2Section;

impl Osc2Section {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Osc 2").size(style::SECTION_LABEL_TEXT_SIZE),
                wavetable_list(
                    "Table",
                    Parameter::Sound(SoundParameter::Osc2Table),
                    params.get_value(SoundParameter::Osc2Table),
                ),
                slider_with_labels(
                    "Wave",
                    Parameter::Sound(SoundParameter::Osc2Wave),
                    params.get_value(SoundParameter::Osc2Wave),
                ),
                slider_with_labels(
                    "Coarse",
                    Parameter::Sound(SoundParameter::Osc2Coarse),
                    params.get_value(SoundParameter::Osc2Coarse),
                ),
                slider_with_labels(
                    "Fine",
                    Parameter::Sound(SoundParameter::Osc2Fine),
                    params.get_value(SoundParameter::Osc2Fine),
                ),
                slider_with_labels(
                    "FM Amt",
                    Parameter::Sound(SoundParameter::Osc2FMAmount),
                    params.get_value(SoundParameter::Osc2FMAmount),
                ),
                slider_with_labels(
                    "FM Rate",
                    Parameter::Sound(SoundParameter::Osc2FMRate),
                    params.get_value(SoundParameter::Osc2FMRate),
                ),
                slider_with_labels(
                    "Sync",
                    Parameter::Sound(SoundParameter::Osc2Sync),
                    params.get_value(SoundParameter::Osc2Sync),
                ),
                slider_with_labels(
                    "Level",
                    Parameter::Sound(SoundParameter::Osc2Level),
                    params.get_value(SoundParameter::Osc2Level),
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xAB, 0xA3, 0x39)))
        .into()
    }
}
