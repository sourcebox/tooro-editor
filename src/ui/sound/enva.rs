//! Section containing the amp envelope parameters

use iced::{
    Color, Element, Length, Padding,
    widget::{column, container, rule, text},
};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues, SoundParameter};
use crate::ui::elements::{
    env_trigger_list::env_trigger_list, mod_target_list::mod_target_list,
    slider::slider_with_labels,
};
use crate::ui::style;

pub struct EnvASection;

impl EnvASection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &ParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Env A").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Attack",
                    Parameter::Sound(SoundParameter::EnvAAttack),
                    params.get_value(Parameter::Sound(SoundParameter::EnvAAttack)),
                ),
                slider_with_labels(
                    "Hold",
                    Parameter::Sound(SoundParameter::EnvAHold),
                    params.get_value(Parameter::Sound(SoundParameter::EnvAHold)),
                ),
                slider_with_labels(
                    "Decay",
                    Parameter::Sound(SoundParameter::EnvADecay),
                    params.get_value(Parameter::Sound(SoundParameter::EnvADecay)),
                ),
                slider_with_labels(
                    "Sustain",
                    Parameter::Sound(SoundParameter::EnvASustain),
                    params.get_value(Parameter::Sound(SoundParameter::EnvASustain)),
                ),
                slider_with_labels(
                    "Release",
                    Parameter::Sound(SoundParameter::EnvARelease),
                    params.get_value(Parameter::Sound(SoundParameter::EnvARelease)),
                ),
                slider_with_labels(
                    "Velo",
                    Parameter::Sound(SoundParameter::EnvAVelo),
                    params.get_value(Parameter::Sound(SoundParameter::EnvAVelo)),
                ),
                slider_with_labels(
                    "After",
                    Parameter::Sound(SoundParameter::EnvAAfter),
                    params.get_value(Parameter::Sound(SoundParameter::EnvAAfter)),
                ),
                env_trigger_list(
                    "Trigger",
                    Parameter::Sound(SoundParameter::EnvATrigger),
                    params.get_value(Parameter::Sound(SoundParameter::EnvATrigger)),
                ),
                rule::horizontal(1).style(|_| style::rule()),
                mod_target_list(
                    "Target",
                    Parameter::Sound(SoundParameter::ModEnvATarget),
                    params.get_value(Parameter::Sound(SoundParameter::ModEnvATarget)),
                ),
                slider_with_labels(
                    "Mod Amt",
                    Parameter::Sound(SoundParameter::ModEnvAAmount),
                    params.get_value(Parameter::Sound(SoundParameter::ModEnvAAmount)),
                )
            ]
            .height(Length::Fill)
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xFF, 0xBD, 0x00)))
        .into()
    }
}
