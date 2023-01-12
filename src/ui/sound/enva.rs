//! Section containing the amp envelope parameters

use iced::widget::{Column, Container, Rule, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{
    env_trigger_list::env_trigger_list, mod_target_list::mod_target_list,
    slider::slider_with_labels,
};
use crate::ui::style;

pub struct EnvASection {}

impl EnvASection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Env A").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Attack",
                SoundParameter::EnvAAttack,
                params.get_value(SoundParameter::EnvAAttack),
            ))
            .push(slider_with_labels(
                "Hold",
                SoundParameter::EnvAHold,
                params.get_value(SoundParameter::EnvAHold),
            ))
            .push(slider_with_labels(
                "Decay",
                SoundParameter::EnvADecay,
                params.get_value(SoundParameter::EnvADecay),
            ))
            .push(slider_with_labels(
                "Sustain",
                SoundParameter::EnvASustain,
                params.get_value(SoundParameter::EnvASustain),
            ))
            .push(slider_with_labels(
                "Release",
                SoundParameter::EnvARelease,
                params.get_value(SoundParameter::EnvARelease),
            ))
            .push(slider_with_labels(
                "Velo",
                SoundParameter::EnvAVelo,
                params.get_value(SoundParameter::EnvAVelo),
            ))
            .push(slider_with_labels(
                "After",
                SoundParameter::EnvAAfter,
                params.get_value(SoundParameter::EnvAAfter),
            ))
            .push(env_trigger_list(
                "Trigger",
                SoundParameter::EnvATrigger,
                params.get_value(SoundParameter::EnvATrigger),
            ))
            .push(Rule::horizontal(10))
            .push(mod_target_list(
                "Mod Target",
                SoundParameter::ModEnvATarget,
                params.get_value(SoundParameter::ModEnvATarget),
            ))
            .push(slider_with_labels(
                "Mod Amt",
                SoundParameter::ModEnvAAmount,
                params.get_value(SoundParameter::ModEnvAAmount),
            ));
        Container::new(content).style(style::EnvSection).into()
    }
}
