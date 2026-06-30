//! Section containing the filter envelope parameters

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

pub struct EnvFSection;

impl EnvFSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&'a self, values: &'a ParameterValues) -> Element<'a, Message> {
        container(
            column![
                text("Env F").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Attack",
                    Parameter::Sound(SoundParameter::EnvFAttack),
                    values
                ),
                slider_with_labels("Hold", Parameter::Sound(SoundParameter::EnvFHold), values),
                slider_with_labels("Decay", Parameter::Sound(SoundParameter::EnvFDecay), values),
                slider_with_labels(
                    "Sustain",
                    Parameter::Sound(SoundParameter::EnvFSustain),
                    values
                ),
                slider_with_labels(
                    "Release",
                    Parameter::Sound(SoundParameter::EnvFRelease),
                    values
                ),
                slider_with_labels("Velo", Parameter::Sound(SoundParameter::EnvFVelo), values),
                slider_with_labels("After", Parameter::Sound(SoundParameter::EnvFAfter), values),
                env_trigger_list(
                    "Trigger",
                    Parameter::Sound(SoundParameter::EnvFTrigger),
                    values
                ),
                rule::horizontal(1).style(|_| style::rule()),
                mod_target_list(
                    "Target",
                    Parameter::Sound(SoundParameter::ModEnvFTarget),
                    values
                ),
                slider_with_labels(
                    "Mod Amt",
                    Parameter::Sound(SoundParameter::ModEnvFAmount),
                    values
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
