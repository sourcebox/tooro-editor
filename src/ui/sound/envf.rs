//! Section containing the filter envelope parameters

use iced::{
    Color, Element, Length, Padding,
    widget::{column, container, rule, text},
};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
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
                    SoundParameter::EnvFAttack,
                    params.get_value(SoundParameter::EnvFAttack),
                ),
                slider_with_labels(
                    "Hold",
                    SoundParameter::EnvFHold,
                    params.get_value(SoundParameter::EnvFHold),
                ),
                slider_with_labels(
                    "Decay",
                    SoundParameter::EnvFDecay,
                    params.get_value(SoundParameter::EnvFDecay),
                ),
                slider_with_labels(
                    "Sustain",
                    SoundParameter::EnvFSustain,
                    params.get_value(SoundParameter::EnvFSustain),
                ),
                slider_with_labels(
                    "Release",
                    SoundParameter::EnvFRelease,
                    params.get_value(SoundParameter::EnvFRelease),
                ),
                slider_with_labels(
                    "Velo",
                    SoundParameter::EnvFVelo,
                    params.get_value(SoundParameter::EnvFVelo),
                ),
                slider_with_labels(
                    "After",
                    SoundParameter::EnvFAfter,
                    params.get_value(SoundParameter::EnvFAfter),
                ),
                env_trigger_list(
                    "Trigger",
                    SoundParameter::EnvFTrigger,
                    params.get_value(SoundParameter::EnvFTrigger),
                ),
                rule::horizontal(1).style(|_| style::rule()),
                mod_target_list(
                    "Target",
                    SoundParameter::ModEnvFTarget,
                    params.get_value(SoundParameter::ModEnvFTarget),
                ),
                slider_with_labels(
                    "Mod Amt",
                    SoundParameter::ModEnvFAmount,
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
