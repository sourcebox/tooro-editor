use iced::{pick_list, slider, Column, Container, Element, Text};

use crate::elements::{
    lfo_phase_list::{lfo_phase_list, LFOPhase},
    lfo_shape_list::{lfo_shape_list, LFOShape},
    slider::slider_with_labels,
};
use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::style;

pub struct LFO2Section {
    shape_list: pick_list::State<LFOShape>,
    speed_slider: slider::State,
    rise_slider: slider::State,
    phase_list: pick_list::State<LFOPhase>,
}

impl LFO2Section {
    pub fn new() -> Self {
        Self {
            shape_list: pick_list::State::<LFOShape>::default(),
            speed_slider: slider::State::new(),
            rise_slider: slider::State::new(),
            phase_list: pick_list::State::<LFOPhase>::default(),
        }
    }

    pub fn view(&mut self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("LFO 2"))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(lfo_shape_list(
                "Shape",
                &mut self.shape_list,
                SoundParameter::LFO2Shape,
                params.get_value(SoundParameter::LFO2Shape),
            ))
            .push(slider_with_labels(
                "Speed",
                &mut self.speed_slider,
                SoundParameter::LFO2Speed,
                params.get_value(SoundParameter::LFO2Speed),
            ))
            .push(slider_with_labels(
                "Rise",
                &mut self.rise_slider,
                SoundParameter::LFO2Rise,
                params.get_value(SoundParameter::LFO2Rise),
            ))
            .push(lfo_phase_list(
                "Phase",
                &mut self.phase_list,
                SoundParameter::LFO2Phase,
                params.get_value(SoundParameter::LFO2Phase),
            ));
        Container::new(content).style(style::LFOSection).into()
    }
}
