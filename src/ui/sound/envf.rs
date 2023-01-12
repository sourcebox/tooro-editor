//! Section containing the filter envelope parameters

use iced::widget::{Column, Container, Rule, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{
    env_trigger_list::env_trigger_list, mod_target_list::mod_target_list,
    slider::slider_with_labels,
};
use crate::ui::style;

pub struct EnvFSection {}

impl EnvFSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Env F").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Attack",
                SoundParameter::EnvFAttack,
                params.get_value(SoundParameter::EnvFAttack),
            ))
            .push(slider_with_labels(
                "Hold",
                SoundParameter::EnvFHold,
                params.get_value(SoundParameter::EnvFHold),
            ))
            .push(slider_with_labels(
                "Decay",
                SoundParameter::EnvFDecay,
                params.get_value(SoundParameter::EnvFDecay),
            ))
            .push(slider_with_labels(
                "Sustain",
                SoundParameter::EnvFSustain,
                params.get_value(SoundParameter::EnvFSustain),
            ))
            .push(slider_with_labels(
                "Release",
                SoundParameter::EnvFRelease,
                params.get_value(SoundParameter::EnvFRelease),
            ))
            .push(slider_with_labels(
                "Velo",
                SoundParameter::EnvFVelo,
                params.get_value(SoundParameter::EnvFVelo),
            ))
            .push(slider_with_labels(
                "After",
                SoundParameter::EnvFAfter,
                params.get_value(SoundParameter::EnvFAfter),
            ))
            .push(env_trigger_list(
                "Trigger",
                SoundParameter::EnvFTrigger,
                params.get_value(SoundParameter::EnvFTrigger),
            ))
            .push(Rule::horizontal(10))
            .push(mod_target_list(
                "Mod Target",
                SoundParameter::ModEnvFTarget,
                params.get_value(SoundParameter::ModEnvFTarget),
            ))
            .push(slider_with_labels(
                "Mod Amt",
                SoundParameter::ModEnvFAmount,
                params.get_value(SoundParameter::ModEnvFAmount),
            ));
        Container::new(content)
            // .style(style::EnvSection)
            .into()
    }
}
