//! Section containing the filter envelope parameters

use iced::{
    Color, Element, Length, Padding,
    widget::{column, container, rule, text},
};

use crate::messages::Message;
use crate::params::{GetValue, Parameter, SoundParameter, SoundParameterValues};
use crate::ui::elements::{
    env_trigger_list::env_trigger_list, mod_target_list::mod_target_list,
    slider::slider_with_labels,
};
use crate::ui::style;

pub struct EnvFSection;

impl EnvFSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Env F").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Attack",
                    Parameter::Sound(SoundParameter::EnvFAttack),
                    params.get_value(SoundParameter::EnvFAttack),
                ),
                slider_with_labels(
                    "Hold",
                    Parameter::Sound(SoundParameter::EnvFHold),
                    params.get_value(SoundParameter::EnvFHold),
                ),
                slider_with_labels(
                    "Decay",
                    Parameter::Sound(SoundParameter::EnvFDecay),
                    params.get_value(SoundParameter::EnvFDecay),
                ),
                slider_with_labels(
                    "Sustain",
                    Parameter::Sound(SoundParameter::EnvFSustain),
                    params.get_value(SoundParameter::EnvFSustain),
                ),
                slider_with_labels(
                    "Release",
                    Parameter::Sound(SoundParameter::EnvFRelease),
                    params.get_value(SoundParameter::EnvFRelease),
                ),
                slider_with_labels(
                    "Velo",
                    Parameter::Sound(SoundParameter::EnvFVelo),
                    params.get_value(SoundParameter::EnvFVelo),
                ),
                slider_with_labels(
                    "After",
                    Parameter::Sound(SoundParameter::EnvFAfter),
                    params.get_value(SoundParameter::EnvFAfter),
                ),
                env_trigger_list(
                    "Trigger",
                    Parameter::Sound(SoundParameter::EnvFTrigger),
                    params.get_value(SoundParameter::EnvFTrigger),
                ),
                rule::horizontal(1).style(|_| style::rule()),
                mod_target_list(
                    "Target",
                    Parameter::Sound(SoundParameter::ModEnvFTarget),
                    params.get_value(SoundParameter::ModEnvFTarget),
                ),
                slider_with_labels(
                    "Mod Amt",
                    Parameter::Sound(SoundParameter::ModEnvFAmount),
                    params.get_value(SoundParameter::ModEnvFAmount),
                ),
            ]
            .height(Length::Fill)
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xFF, 0xBD, 0x00)))
        .into()
    }
}
