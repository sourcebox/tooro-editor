//! Section containing the LFO 1 parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, rule, text},
};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues, SoundParameter};
use crate::ui::elements::{
    lfo_phase_list::lfo_phase_list, lfo_shape_list::lfo_shape_list,
    mod_target_list::mod_target_list, slider::slider_with_labels,
};
use crate::ui::style;

pub struct LFO1Section;

impl LFO1Section {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &ParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("LFO 1").size(style::SECTION_LABEL_TEXT_SIZE),
                lfo_shape_list(
                    "Shape",
                    Parameter::Sound(SoundParameter::LFO1Shape),
                    params.get_value(Parameter::Sound(SoundParameter::LFO1Shape)),
                ),
                slider_with_labels(
                    "Speed",
                    Parameter::Sound(SoundParameter::LFO1Speed),
                    params.get_value(Parameter::Sound(SoundParameter::LFO1Speed)),
                ),
                slider_with_labels(
                    "Rise",
                    Parameter::Sound(SoundParameter::LFO1Rise),
                    params.get_value(Parameter::Sound(SoundParameter::LFO1Rise)),
                ),
                lfo_phase_list(
                    "Phase",
                    Parameter::Sound(SoundParameter::LFO1Phase),
                    params.get_value(Parameter::Sound(SoundParameter::LFO1Phase)),
                ),
                rule::horizontal(1).style(|_| style::rule()),
                mod_target_list(
                    "Target",
                    Parameter::Sound(SoundParameter::ModLFO1Target),
                    params.get_value(Parameter::Sound(SoundParameter::ModLFO1Target)),
                ),
                slider_with_labels(
                    "Mod Amt",
                    Parameter::Sound(SoundParameter::ModLFO1Amount),
                    params.get_value(Parameter::Sound(SoundParameter::ModLFO1Amount)),
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xD2, 0x6A, 0x25)))
        .into()
    }
}
