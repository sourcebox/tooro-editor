//! Section containing the amp envelope parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, rule, text},
};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
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

    pub fn view(&self, params: &SoundParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Env A").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Attack",
                    SoundParameter::EnvAAttack,
                    params.get_value(SoundParameter::EnvAAttack),
                ),
                slider_with_labels(
                    "Hold",
                    SoundParameter::EnvAHold,
                    params.get_value(SoundParameter::EnvAHold),
                ),
                slider_with_labels(
                    "Decay",
                    SoundParameter::EnvADecay,
                    params.get_value(SoundParameter::EnvADecay),
                ),
                slider_with_labels(
                    "Sustain",
                    SoundParameter::EnvASustain,
                    params.get_value(SoundParameter::EnvASustain),
                ),
                slider_with_labels(
                    "Release",
                    SoundParameter::EnvARelease,
                    params.get_value(SoundParameter::EnvARelease),
                ),
                slider_with_labels(
                    "Velo",
                    SoundParameter::EnvAVelo,
                    params.get_value(SoundParameter::EnvAVelo),
                ),
                slider_with_labels(
                    "After",
                    SoundParameter::EnvAAfter,
                    params.get_value(SoundParameter::EnvAAfter),
                ),
                env_trigger_list(
                    "Trigger",
                    SoundParameter::EnvATrigger,
                    params.get_value(SoundParameter::EnvATrigger),
                ),
                rule::horizontal(1).style(|_| style::rule()),
                mod_target_list(
                    "Target",
                    SoundParameter::ModEnvATarget,
                    params.get_value(SoundParameter::ModEnvATarget),
                ),
                slider_with_labels(
                    "Mod Amt",
                    SoundParameter::ModEnvAAmount,
                    params.get_value(SoundParameter::ModEnvAAmount),
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xFF, 0xBD, 0x00)))
        .into()
    }
}
