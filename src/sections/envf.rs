use iced::{pick_list, slider, Column, Container, Element, Text};

use crate::elements::{
    env_trigger_list::{env_trigger_list, EnvTrigger},
    slider::slider_with_labels,
};
use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::style;

pub struct EnvFSection {
    attack_slider: slider::State,
    decay_slider: slider::State,
    sustain_slider: slider::State,
    release_slider: slider::State,
    velo_slider: slider::State,
    hold_slider: slider::State,
    after_slider: slider::State,
    trigger_list: pick_list::State<EnvTrigger>,
}

impl EnvFSection {
    pub fn new() -> Self {
        Self {
            attack_slider: slider::State::new(),
            decay_slider: slider::State::new(),
            sustain_slider: slider::State::new(),
            release_slider: slider::State::new(),
            velo_slider: slider::State::new(),
            hold_slider: slider::State::new(),
            after_slider: slider::State::new(),
            trigger_list: pick_list::State::<EnvTrigger>::default(),
        }
    }

    pub fn view(&mut self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Env F"))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Attack",
                &mut self.attack_slider,
                SoundParameter::EnvFAttack,
                params.get_value(SoundParameter::EnvFAttack),
            ))
            .push(slider_with_labels(
                "Hold",
                &mut self.hold_slider,
                SoundParameter::EnvFHold,
                params.get_value(SoundParameter::EnvFHold),
            ))
            .push(slider_with_labels(
                "Decay",
                &mut self.decay_slider,
                SoundParameter::EnvFDecay,
                params.get_value(SoundParameter::EnvFDecay),
            ))
            .push(slider_with_labels(
                "Sustain",
                &mut self.sustain_slider,
                SoundParameter::EnvFSustain,
                params.get_value(SoundParameter::EnvFSustain),
            ))
            .push(slider_with_labels(
                "Release",
                &mut self.release_slider,
                SoundParameter::EnvFRelease,
                params.get_value(SoundParameter::EnvFRelease),
            ))
            .push(slider_with_labels(
                "Velo",
                &mut self.velo_slider,
                SoundParameter::EnvFVelo,
                params.get_value(SoundParameter::EnvFVelo),
            ))
            .push(slider_with_labels(
                "After",
                &mut self.after_slider,
                SoundParameter::EnvFAfter,
                params.get_value(SoundParameter::EnvFAfter),
            ))
            .push(env_trigger_list(
                "Trigger",
                &mut self.trigger_list,
                SoundParameter::EnvFTrigger,
                params.get_value(SoundParameter::EnvFTrigger),
            ));
        Container::new(content).style(style::EnvSection).into()
    }
}
