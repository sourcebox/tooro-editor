//! Section containing oscillator 1 parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues, SoundParameter};
use crate::ui::elements::{slider::slider_with_labels, wavetable_list::wavetable_list};
use crate::ui::style;

pub struct Osc1Section;

impl Osc1Section {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &ParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Osc 1").size(style::SECTION_LABEL_TEXT_SIZE),
                wavetable_list(
                    "Table",
                    Parameter::Sound(SoundParameter::Osc1Table),
                    params.get_value(Parameter::Sound(SoundParameter::Osc1Table)),
                ),
                slider_with_labels(
                    "Wave",
                    Parameter::Sound(SoundParameter::Osc1Wave),
                    params.get_value(Parameter::Sound(SoundParameter::Osc1Wave)),
                ),
                slider_with_labels(
                    "Coarse",
                    Parameter::Sound(SoundParameter::Osc1Coarse),
                    params.get_value(Parameter::Sound(SoundParameter::Osc1Coarse)),
                ),
                slider_with_labels(
                    "Fine",
                    Parameter::Sound(SoundParameter::Osc1Fine),
                    params.get_value(Parameter::Sound(SoundParameter::Osc1Fine)),
                ),
                slider_with_labels(
                    "FM Amt",
                    Parameter::Sound(SoundParameter::Osc1FMAmount),
                    params.get_value(Parameter::Sound(SoundParameter::Osc1FMAmount)),
                ),
                slider_with_labels(
                    "FM Rate",
                    Parameter::Sound(SoundParameter::Osc1FMRate),
                    params.get_value(Parameter::Sound(SoundParameter::Osc1FMRate)),
                ),
                slider_with_labels(
                    "Sync",
                    Parameter::Sound(SoundParameter::Osc1Sync),
                    params.get_value(Parameter::Sound(SoundParameter::Osc1Sync)),
                ),
                slider_with_labels(
                    "Level",
                    Parameter::Sound(SoundParameter::Osc1Level),
                    params.get_value(Parameter::Sound(SoundParameter::Osc1Level)),
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xAB, 0xA3, 0x39)))
        .into()
    }
}
