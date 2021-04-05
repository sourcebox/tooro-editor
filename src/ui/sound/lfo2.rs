use iced::{pick_list, slider, Column, Container, Element, Rule, Text};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{
    lfo_phase_list::{lfo_phase_list, LFOPhase},
    lfo_shape_list::{lfo_shape_list, LFOShape},
    mod_target_list::{mod_target_list, ModTarget},
    slider::slider_with_labels,
};
use crate::ui::style;

pub struct LFO2Section {
    shape_list: pick_list::State<LFOShape>,
    speed_slider: slider::State,
    rise_slider: slider::State,
    phase_list: pick_list::State<LFOPhase>,
    mod_target_list: pick_list::State<ModTarget>,
    mod_amount_slider: slider::State,
}

impl LFO2Section {
    pub fn new() -> Self {
        Self {
            shape_list: pick_list::State::<LFOShape>::default(),
            speed_slider: slider::State::new(),
            rise_slider: slider::State::new(),
            phase_list: pick_list::State::<LFOPhase>::default(),
            mod_target_list: pick_list::State::<ModTarget>::default(),
            mod_amount_slider: slider::State::new(),
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
            ))
            .push(Rule::horizontal(10))
            .push(mod_target_list(
                "Mod Target",
                &mut self.mod_target_list,
                SoundParameter::ModLFO2Target,
                params.get_value(SoundParameter::ModLFO2Target),
            ))
            .push(slider_with_labels(
                "Mod Amount",
                &mut self.mod_amount_slider,
                SoundParameter::ModLFO2Amount,
                params.get_value(SoundParameter::ModLFO2Amount),
            ));
        Container::new(content).style(style::LFOSection).into()
    }
}
