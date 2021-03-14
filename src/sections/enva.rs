use iced::{pick_list, slider, Column, Container, Element, Text};

use crate::elements::{
    env_trigger_list::{env_trigger_list, EnvTrigger},
    slider::slider_with_labels,
};
use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::style;

pub struct EnvASection {
    attack_slider: slider::State,
    decay_slider: slider::State,
    sustain_slider: slider::State,
    release_slider: slider::State,
    velo_slider: slider::State,
    hold_slider: slider::State,
    after_slider: slider::State,
    trigger_list: pick_list::State<EnvTrigger>,
}

impl EnvASection {
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
            .push(Text::new("Env A"))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Attack",
                &mut self.attack_slider,
                SoundParameter::EnvAAttack,
                params.get_value(SoundParameter::EnvAAttack),
            ))
            .push(slider_with_labels(
                "Hold",
                &mut self.hold_slider,
                SoundParameter::EnvAHold,
                params.get_value(SoundParameter::EnvAHold),
            ))
            .push(slider_with_labels(
                "Decay",
                &mut self.decay_slider,
                SoundParameter::EnvADecay,
                params.get_value(SoundParameter::EnvADecay),
            ))
            .push(slider_with_labels(
                "Sustain",
                &mut self.sustain_slider,
                SoundParameter::EnvASustain,
                params.get_value(SoundParameter::EnvASustain),
            ))
            .push(slider_with_labels(
                "Release",
                &mut self.release_slider,
                SoundParameter::EnvARelease,
                params.get_value(SoundParameter::EnvARelease),
            ))
            .push(slider_with_labels(
                "Velo",
                &mut self.velo_slider,
                SoundParameter::EnvAVelo,
                params.get_value(SoundParameter::EnvAVelo),
            ))
            .push(slider_with_labels(
                "After",
                &mut self.after_slider,
                SoundParameter::EnvAAfter,
                params.get_value(SoundParameter::EnvAAfter),
            ))
            .push(env_trigger_list(
                "Trigger",
                &mut self.trigger_list,
                SoundParameter::EnvATrigger,
                params.get_value(SoundParameter::EnvATrigger),
            ));
        Container::new(content).style(style::EnvSection).into()
    }
}
