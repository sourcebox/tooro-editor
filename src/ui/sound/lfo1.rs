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

pub struct LFO1Section {
    shape_list: pick_list::State<LFOShape>,
    speed_slider: slider::State,
    rise_slider: slider::State,
    phase_list: pick_list::State<LFOPhase>,
    mod_target_list: pick_list::State<ModTarget>,
    mod_amount_slider: slider::State,
}

impl LFO1Section {
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
            .push(Text::new("LFO 1").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(lfo_shape_list(
                "Shape",
                &mut self.shape_list,
                SoundParameter::LFO1Shape,
                params.get_value(SoundParameter::LFO1Shape),
            ))
            .push(slider_with_labels(
                "Speed",
                &mut self.speed_slider,
                SoundParameter::LFO1Speed,
                params.get_value(SoundParameter::LFO1Speed),
            ))
            .push(slider_with_labels(
                "Rise",
                &mut self.rise_slider,
                SoundParameter::LFO1Rise,
                params.get_value(SoundParameter::LFO1Rise),
            ))
            .push(lfo_phase_list(
                "Phase",
                &mut self.phase_list,
                SoundParameter::LFO1Phase,
                params.get_value(SoundParameter::LFO1Phase),
            ))
            .push(Rule::horizontal(10))
            .push(mod_target_list(
                "Mod Target",
                &mut self.mod_target_list,
                SoundParameter::ModLFO1Target,
                params.get_value(SoundParameter::ModLFO1Target),
            ))
            .push(slider_with_labels(
                "Mod Amount",
                &mut self.mod_amount_slider,
                SoundParameter::ModLFO1Amount,
                params.get_value(SoundParameter::ModLFO1Amount),
            ));
        Container::new(content).style(style::LFOSection).into()
    }
}
