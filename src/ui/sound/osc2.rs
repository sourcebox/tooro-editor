//! Section containing oscillator 2 parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues, SoundParameter};
use crate::ui::elements::{slider::slider_with_labels, wavetable_list::wavetable_list};
use crate::ui::style;

pub struct Osc2Section;

impl Osc2Section {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&'a self, values: &'a ParameterValues) -> Element<'a, Message> {
        container(
            column![
                text("Osc 2").size(style::SECTION_LABEL_TEXT_SIZE),
                wavetable_list("Table", Parameter::Sound(SoundParameter::Osc2Table), values),
                slider_with_labels("Wave", Parameter::Sound(SoundParameter::Osc2Wave), values),
                slider_with_labels(
                    "Coarse",
                    Parameter::Sound(SoundParameter::Osc2Coarse),
                    values
                ),
                slider_with_labels("Fine", Parameter::Sound(SoundParameter::Osc2Fine), values),
                slider_with_labels(
                    "FM Amt",
                    Parameter::Sound(SoundParameter::Osc2FMAmount),
                    values
                ),
                slider_with_labels(
                    "FM Rate",
                    Parameter::Sound(SoundParameter::Osc2FMRate),
                    values
                ),
                slider_with_labels("Sync", Parameter::Sound(SoundParameter::Osc2Sync), values),
                slider_with_labels("Level", Parameter::Sound(SoundParameter::Osc2Level), values)
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xAB, 0xA3, 0x39)))
        .into()
    }
}
